use crate::core::color::Color;
use crate::core::hit::Hittable;
use crate::core::math::interval::Interval;
use crate::core::math::vector::{Point, UnitVec3D, Vec3D, VecLike};
use crate::core::math::{self, Real};
use crate::core::ray::Ray;
use crate::diagnostics::stats;
use crate::generate_optional_setter;
use crate::settings::Config;
use std::borrow::Cow;
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

    background: Color,
}

impl Camera {
    pub fn builder(config: &'static Config) -> CameraBuilder {
        CameraBuilder::new(config)
    }

    pub fn render(&self, world: &Hittable, config: &Config) -> io::Result<()> {
        let start = Instant::now();
        let ppm_file = File::create(config.app().scene().output_file())?;
        let viewport = self.viewport();
        let pixel_sample_scale = self.pixel_sample_scale();

        stats::report(world);

        // output the PPM contents
        writeln!(
            &ppm_file,
            "P3\n{} {}\n255",
            self.image.width,
            self.image.height()
        )?;

        let mut ppm_content = String::new();

        for j in 0..self.image.height() {
            println!("Scanlines remaining: {}", self.image.height() - j);

            for i in 0..self.image.width {
                let pixel_color = if self.antialiasing {
                    // compute the average color from the sample rays
                    (0..self.samples_per_pixel)
                        .map(|_| {
                            self.ray_color(&self.get_ray(i, j, &viewport), self.max_depth, world)
                        })
                        .fold(Color::black(), |acc, color| acc + color)
                        * pixel_sample_scale
                } else {
                    let pixel_center = viewport.pixel_00_loc()
                        + (viewport.pixel_delta_horizontal() * i as Real)
                        + (viewport.pixel_delta_vertical() * j as Real);
                    let ray_direction = pixel_center - self.center();
                    let ray = Ray::from_ref_origin(self.center(), ray_direction);
                    self.ray_color(&ray, self.max_depth, world)
                };

                ppm_content += &format!("{}\n", pixel_color.to_bytes_string());
            }
        }

        writeln!(&ppm_file, "{}", ppm_content)?;

        let duration = start.elapsed();
        println!("Done. Rendering time: {:?}", duration);

        Ok(())
    }

    /// Returns a ray directed towards a randomly sampled point around the pixel at i, j
    fn get_ray(&self, i: u32, j: u32, viewport: &Viewport) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = viewport.pixel_00_loc()
            + (viewport.pixel_delta_horizontal() * (offset.x + i as Real))
            + (viewport.pixel_delta_vertical() * (offset.y + j as Real));
        let origin = if self.defocus_angle <= 0.0 {
            Cow::Borrowed(self.center())
        } else {
            Cow::Owned(self.defocus_disk_sample())
        };
        let direction = pixel_sample - origin.as_ref();
        let ray_time = math::random_real();

        Ray::timed(origin, direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = Vec3D::random_in_unit_disk();
        self.center() + (&self.defocus_disk.horizontal * p.x) + (&self.defocus_disk.vertical * p.y)
    }

    /// A vector to a random point within half the unit square.
    fn sample_square() -> Vec3D {
        Vec3D::new(math::random_real() - 0.5, math::random_real() - 0.5, 0.0)
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &Hittable) -> Color {
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
            self.background.clone()
        }
    }

    fn viewport(&self) -> Viewport {
        let theta = math::degrees_to_radians(self.field_of_view);
        let h = Real::tan(theta / 2.0);
        let height = 2.0 * h * self.focus_distance;
        Viewport::new(height, &self, &self.image)
    }

    fn pixel_sample_scale(&self) -> Real {
        1.0 / self.samples_per_pixel as Real
    }

    fn center(&self) -> &Point {
        &self.look_from
    }
}

pub struct CameraBuilder {
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
    background: Option<Color>,
    config: &'static Config,
}

impl CameraBuilder {
    fn new(config: &'static Config) -> CameraBuilder {
        CameraBuilder {
            image: None,
            samples_per_pixel: None,
            antialiasing: None,
            max_depth: None,
            field_of_view: None,
            look_from: None,
            look_at: None,
            vup: None,
            defocus_angle: None,
            focus_distance: None,
            background: None,
            config,
        }
    }

    generate_optional_setter!(image, Image);
    generate_optional_setter!(samples_per_pixel, u32);
    generate_optional_setter!(antialiasing, bool);
    generate_optional_setter!(max_depth, u32);
    generate_optional_setter!(field_of_view, Real);
    generate_optional_setter!(look_from, Point);
    generate_optional_setter!(look_at, Point);
    generate_optional_setter!(vup, Vec3D);
    generate_optional_setter!(defocus_angle, Real);
    generate_optional_setter!(focus_distance, Real);
    generate_optional_setter!(background, Color);

    pub fn build(&self) -> Camera {
        fn build_vec_like<K>(p: [Real; 3]) -> VecLike<K> {
            VecLike::new(p[0], p[1], p[2])
        }

        let defaults = self.config.app().scene().camera().defaults();
        let look_from = self
            .look_from
            .clone()
            .unwrap_or(build_vec_like(defaults.look_from()));
        let look_at = self
            .look_at
            .clone()
            .unwrap_or(build_vec_like(defaults.look_at()));
        let looks_delta = &look_from - &look_at;
        let vup = self.vup.clone().unwrap_or(build_vec_like(defaults.vup()));

        let out = looks_delta.to_unit();
        let right = vup.cross(&out.0).to_unit();

        // technically, there's no need to normalize because it's a cross-product
        // of two perpendicular unit vectors
        let up = UnitVec3D(out.0.cross(&right.0));

        let mut camera = Camera {
            image: self.image.clone().unwrap_or(Image::new(100, 1.0)),
            samples_per_pixel: self
                .samples_per_pixel
                .unwrap_or(defaults.samples_per_pixel()),
            antialiasing: self.antialiasing.unwrap_or(defaults.antialiasing()),
            max_depth: self.max_depth.unwrap_or(defaults.max_depth()),
            field_of_view: self.field_of_view.unwrap_or(defaults.field_of_view()),
            look_from,
            look_at,
            vup,
            out,
            right,
            up,
            defocus_angle: self.defocus_angle.unwrap_or(defaults.defocus_angle()),
            focus_distance: self.focus_distance.unwrap_or(defaults.focus_distance()),
            defocus_disk: DefocusDisk::empty(),
            background: self
                .background
                .clone()
                .unwrap_or(build_vec_like(defaults.background())),
        };

        camera.defocus_disk = DefocusDisk::from_camera(&camera);
        camera
    }
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
