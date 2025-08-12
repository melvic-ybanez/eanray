use crate::core::color::Color;
use crate::core::hittables::{Hittable, World};
use crate::core::math::interval::Interval;
use crate::core::math::vector::{Point, UnitVec3D, Vec3D, VecLike};
use crate::core::math::{self, Real};
use crate::core::ray::Ray;
use crate::diagnostics::stats;
use crate::generate_optional_setter;
use crate::settings::Config;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::time::Instant;

pub struct Camera {
    image: Image,
    samples_per_pixel: u32,
    antialiasing: bool,
    max_depth: u32,
    field_of_view: Real,
    look_from: Point,
    look_at: Point,
    vup: Vec3D,

    // basis vectors
    right: UnitVec3D,
    up: UnitVec3D,
    out: UnitVec3D,

    defocus_angle: Real,
    focus_distance: Real,

    defocus_disk: DefocusDisk,

    background: Background,

    tile_width: u32,
    tile_height: u32,
}

impl Camera {
    pub fn builder(config: &'static Config) -> CameraBuilder {
        CameraBuilder::new(config)
    }

    pub fn render(&self, world: &World, config: &Config) -> io::Result<()> {
        let start = Instant::now();
        let ppm_file = File::create(config.app().scene().output_file())?;

        world.bvh().into_iter().for_each(stats::report_bvh);
        stats::report(world.raws());

        // output the PPM contents
        writeln!(
            &ppm_file,
            "P3\n{} {}\n255",
            self.image.width,
            self.image.height()
        )?;

        let ppm_content = self.ppm_content(world);
        writeln!(&ppm_file, "{}", ppm_content)?;

        let duration = start.elapsed();
        log::info!("Rendering complete. Duration: {:?}", duration);

        Ok(())
    }

    fn ppm_content(&self, world: &World) -> String {
        let viewport = self.viewport();
        let pixel_sample_scale = self.pixel_sample_scale();

        log::info!("Tile size: {} x {}", self.tile_width, self.tile_height);

        log::info!("Splitting the screen into multiple tiles...");
        let tiles: Vec<(u32, u32)> = (0..self.image.height)
            .step_by(self.tile_height as usize)
            .flat_map(|y| {
                (0..self.image.width)
                    .step_by(self.tile_width as usize)
                    .map(move |x| (x, y))
            })
            .collect();

        log::info!("Rendering {} tiles...", tiles.len());
        let pixel_tiles = tiles
            .into_par_iter()
            .fold(
                || Vec::new(),
                |mut local_buffer, (x, y)| {
                    for j in y..(y + self.tile_height).min(self.image.height) {
                        for i in x..(x + self.tile_width).min(self.image.width) {
                            let pixel_color =
                                self.pixel_color(i, j, pixel_sample_scale, &viewport, world);
                            local_buffer.push((i, j, pixel_color));
                        }
                    }
                    log::info!("Tile {x}, {y} rendering complete.");
                    local_buffer
                },
            )
            .reduce(
                || Vec::new(),
                |mut acc, buffer| {
                    acc.extend(buffer);
                    acc
                },
            );

        log::info!("Merging tiles into one buffer...");
        let mut pixels: Vec<Vec<String>> =
            vec![vec![String::new(); self.image.width as usize]; self.image.height as usize];

        for (x, y, color) in pixel_tiles {
            pixels[y as usize][x as usize] = format!("{} ", color.to_bytes_string());
        }
        pixels
            .iter()
            .map(|row| row.join(""))
            .collect::<Vec<_>>()
            .join("")
    }

    fn pixel_color(
        &self,
        i: u32,
        j: u32,
        pixel_sample_scale: f64,
        viewport: &Viewport,
        world: &World,
    ) -> Color {
        if self.antialiasing {
            // compute the average color from the sample rays
            (0..self.samples_per_pixel)
                .map(|_| self.ray_color(&self.get_ray(i, j, &viewport), self.max_depth, world))
                .fold(Color::black(), |acc, color| acc + color)
                * pixel_sample_scale
        } else {
            let pixel_center = viewport.pixel_00_loc()
                + (viewport.pixel_delta_horizontal() * i as Real)
                + (viewport.pixel_delta_vertical() * j as Real);
            let ray_direction = pixel_center - self.center();
            let ray = Ray::new(self.center().clone(), ray_direction);
            self.ray_color(&ray, self.max_depth, world)
        }
    }

    /// Returns a ray directed towards a randomly sampled point around the pixel at i, j
    fn get_ray(&self, i: u32, j: u32, viewport: &Viewport) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = viewport.pixel_00_loc()
            + (viewport.pixel_delta_horizontal() * (offset.x + i as Real))
            + (viewport.pixel_delta_vertical() * (offset.y + j as Real));
        let origin = if self.defocus_angle <= 0.0 {
            self.center().clone()
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - &origin;
        let ray_time = math::random_real();

        Ray::new_timed(origin, direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = Vec3D::random_in_unit_disk();
        self.center() + (&self.defocus_disk.horizontal * p.x) + (&self.defocus_disk.vertical * p.y)
    }

    /// A vector to a random point within half the unit square.
    fn sample_square() -> Vec3D {
        Vec3D::new(math::random_real() - 0.5, math::random_real() - 0.5, 0.0)
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &World) -> Color {
        if depth <= 0 {
            Color::black()
        } else if let Some(record) = world.hit(ray, &mut Interval::new(0.001, math::INFINITY)) {
            let color_from_emission = record
                .material()
                .emitted(record.u(), record.v(), record.p());

            if let Some((scattered, attenuation)) = record.material().scatter(ray, &record) {
                let color_from_scatter = self.ray_color(&scattered, depth - 1, world) * attenuation;
                color_from_emission + color_from_scatter
            } else {
                color_from_emission
            }
        } else {
            match &self.background {
                Background::Color(color) => color.clone(),
                Background::Lerp { start, end } => {
                    let unit_direction = ray.direction().to_unit().0;
                    let a = math::normalize_to_01(unit_direction.y);
                    math::lerp(start, end, a)
                }
            }
        }
    }

    fn viewport(&self) -> Viewport {
        let theta = math::degrees_to_radians(self.field_of_view);
        let h = Real::tan(theta / 2.0);
        let height = 2.0 * h * self.focus_distance;
        Viewport::new(height, &self, &self.image)
    }

    pub fn pixel_sample_scale(&self) -> Real {
        1.0 / self.samples_per_pixel as Real
    }

    fn center(&self) -> &Point {
        &self.look_from
    }
}

pub struct CameraBuilder {
    optionals: OptionalFields,
    config: &'static Config,
}

impl CameraBuilder {
    fn new(config: &'static Config) -> CameraBuilder {
        CameraBuilder {
            optionals: Default::default(),
            config,
        }
    }

    pub fn optionals(&self) -> &OptionalFields {
        &self.optionals
    }

    pub fn build(&self) -> Camera {
        fn build_vec_like<K>(p: [Real; 3]) -> VecLike<K> {
            VecLike::new(p[0], p[1], p[2])
        }

        let optionals = self.optionals();

        let defaults = self.config.app().scene().camera().defaults();
        let look_from = optionals
            .look_from
            .clone()
            .unwrap_or(build_vec_like(defaults.look_from()));
        let look_at = optionals
            .look_at
            .clone()
            .unwrap_or(build_vec_like(defaults.look_at()));
        let looks_delta = &look_from - &look_at;
        let vup = optionals
            .vup
            .clone()
            .unwrap_or(build_vec_like(defaults.vup()));

        let out = looks_delta.to_unit();
        let right = vup.cross(&out.0).to_unit();

        // technically, there's no need to normalize because it's a cross-product
        // of two perpendicular unit vectors
        let up = UnitVec3D(out.0.cross(&right.0));

        let mut camera = Camera {
            image: optionals.image.clone().unwrap_or(Image::new(100, 1.0)),
            samples_per_pixel: optionals
                .samples_per_pixel
                .unwrap_or(defaults.samples_per_pixel()),
            antialiasing: optionals.antialiasing.unwrap_or(defaults.antialiasing()),
            max_depth: optionals.max_depth.unwrap_or(defaults.max_depth()),
            field_of_view: optionals.field_of_view.unwrap_or(defaults.field_of_view()),
            look_from,
            look_at,
            vup,
            out,
            right,
            up,
            defocus_angle: optionals.defocus_angle.unwrap_or(defaults.defocus_angle()),
            focus_distance: optionals
                .focus_distance
                .unwrap_or(defaults.focus_distance()),
            defocus_disk: DefocusDisk::empty(),
            background: optionals
                .background
                .clone()
                .unwrap_or(Background::from_color(build_vec_like(
                    defaults.background(),
                ))),
            tile_width: optionals.tile_width.unwrap_or(defaults.tile_width()),
            tile_height: optionals.tile_height.unwrap_or(defaults.tile_height()),
        };

        camera.defocus_disk = DefocusDisk::from_camera(&camera);
        camera
    }

    generate_optional_setter!(optionals, image, Image);
    generate_optional_setter!(optionals, samples_per_pixel, u32);
    generate_optional_setter!(optionals, antialiasing, bool);
    generate_optional_setter!(optionals, max_depth, u32);
    generate_optional_setter!(optionals, field_of_view, Real);
    generate_optional_setter!(optionals, look_from, Point);
    generate_optional_setter!(optionals, look_at, Point);
    generate_optional_setter!(optionals, vup, Vec3D);
    generate_optional_setter!(optionals, defocus_angle, Real);
    generate_optional_setter!(optionals, focus_distance, Real);
    generate_optional_setter!(optionals, background, Background);
}

#[derive(Default)]
pub struct OptionalFields {
    image: Option<Image>,
    samples_per_pixel: Option<u32>,
    antialiasing: Option<bool>,
    max_depth: Option<u32>,
    field_of_view: Option<Real>,
    look_from: Option<Point>,
    look_at: Option<Point>,
    vup: Option<Vec3D>,
    defocus_angle: Option<Real>,
    focus_distance: Option<Real>,
    background: Option<Background>,
    tile_width: Option<u32>,
    tile_height: Option<u32>,
}

#[derive(Clone)]
pub struct Image {
    width: u32,
    height: u32,

    // Ideal aspect ratio, not the actual one.
    aspect_ratio: f64,
    actual_aspect_ratio: f64,
}

impl Image {
    pub fn new(width: u32, aspect_ratio: f64) -> Image {
        let height = (width as f64 / aspect_ratio) as u32;
        let height = if height < 1 { 1 } else { height };

        Image {
            width,
            height,
            aspect_ratio,
            actual_aspect_ratio: width as f64 / height as f64,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn ideal_aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    /// The actual aspect ratio can be bigger than [`self.aspect_ratio`]
    /// because [`self.height`] truncates decimal points. [`self.height`] also
    /// isn't allowed to have a value of less than 1.
    pub fn actual_aspect_ratio(&self) -> f64 {
        self.actual_aspect_ratio
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

#[derive(Clone)]
struct Viewport {
    height: f64,
    width: f64,
    left_to_right: Vec3D,
    top_to_bottom: Vec3D,
    pixel_delta_horizontal: Vec3D,
    pixel_delta_vertical: Vec3D,
    upper_left: Point,
    pixel_00_loc: Point,
}

impl Viewport {
    pub fn new(height: f64, camera: &Camera, image: &Image) -> Viewport {
        let width = height * image.actual_aspect_ratio();
        let left_to_right = width * &camera.right.0;
        let top_to_bottom = height * &(-&camera.up.0);
        let pixel_delta_horizontal = &left_to_right / image.width as Real;
        let pixel_delta_vertical = &top_to_bottom / image.height as Real;
        let upper_left = camera.center()
            - (camera.focus_distance * &camera.out.0)
            - &left_to_right / 2.0
            - &top_to_bottom / 2.0;
        let pixel_00_loc = &upper_left + (&pixel_delta_horizontal + &pixel_delta_vertical) * 0.5;

        Viewport {
            height,
            width,
            left_to_right,
            top_to_bottom,
            pixel_delta_horizontal,
            pixel_delta_vertical,
            upper_left,
            pixel_00_loc,
        }
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn left_to_right(&self) -> &Vec3D {
        &self.left_to_right
    }

    pub fn top_to_bottom(&self) -> &Vec3D {
        &self.top_to_bottom
    }

    pub fn pixel_delta_horizontal(&self) -> &Vec3D {
        &self.pixel_delta_horizontal
    }

    pub fn pixel_delta_vertical(&self) -> &Vec3D {
        &self.pixel_delta_vertical
    }

    pub fn upper_left(&self) -> &Point {
        &self.upper_left
    }

    pub fn pixel_00_loc(&self) -> &Point {
        &self.pixel_00_loc
    }
}

struct DefocusDisk {
    horizontal: Vec3D,
    vertical: Vec3D,
}

impl DefocusDisk {
    fn empty() -> Self {
        Self {
            horizontal: Vec3D::zero(),
            vertical: Vec3D::zero(),
        }
    }

    fn from_camera(camera: &Camera) -> Self {
        let defocus_radius =
            camera.focus_distance * math::degrees_to_radians(camera.defocus_angle / 2.0).tan();

        Self {
            horizontal: &camera.right.0 * defocus_radius,
            vertical: &camera.up.0 * defocus_radius,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Background {
    Color(Color),
    Lerp { start: Color, end: Color },
}

impl Background {
    pub fn from_color(color: Color) -> Self {
        Self::Color(color)
    }

    pub fn from_lerp(start: Color, end: Color) -> Self {
        Self::Lerp { start, end }
    }
}
