use crate::core::color::Color;
use crate::core::hit::Hittable;
use crate::core::math::interval::Interval;
use crate::core::math::vector::{Point, Vec3D};
use crate::core::math::{self, Real};
use crate::core::ray::Ray;
use crate::settings::Config;
use std::fs::File;
use std::io::{self, Write};
use std::time::Instant;

pub struct Camera {
    center: Point,
    focal_length: f64,
    image: Image,
    samples_per_pixel: u32,
    antialiasing: bool,

    // for now, let's toggle it rather than accept and provide a numerical value
    diffuse: bool,

    max_depth: u32,
}

impl Camera {
    pub fn builder(config: Config) -> CameraBuilder {
        CameraBuilder::new(config)
    }

    pub fn render(&self, world: Hittable) -> io::Result<()> {
        let start = Instant::now();
        let ppm_file = File::create("output.ppm")?;
        let viewport = self.viewport();
        let pixel_sample_scale = self.pixel_sample_scale();

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
                            self.ray_color(&self.get_ray(i, j, &viewport), self.max_depth, &world)
                        })
                        .fold(Color::black(), |acc, color| acc + color)
                        * pixel_sample_scale
                } else {
                    let pixel_center = viewport.pixel_00_loc()
                        + (viewport.pixel_delta_u() * i as Real)
                        + (viewport.pixel_delta_v() * j as Real);
                    let ray_direction = pixel_center - &self.center;
                    let ray = Ray::new(self.center.clone(), ray_direction);
                    self.ray_color(&ray, self.max_depth, &world)
                };

                ppm_content += &format!("{}\n", pixel_color.to_bytes_string());
            }
        }

        writeln!(&ppm_file, "{}", ppm_content)?;

        let duration = start.elapsed();
        println!("Done. Running time: {:?}", duration);

        Ok(())
    }

    /// Returns a ray directed towards a randomly sampled point around the pixel at i, j
    fn get_ray(&self, i: u32, j: u32, viewport: &Viewport) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = viewport.pixel_00_loc()
            + (viewport.pixel_delta_u() * (offset.x + i as Real))
            + (viewport.pixel_delta_v() * (offset.y + j as Real));
        let origin = &self.center;
        Ray::new(origin.clone(), pixel_sample - origin)
    }

    /// A vector to a random point within half the unit square.
    fn sample_square() -> Vec3D {
        Vec3D::new(math::random() - 0.5, math::random() - 0.5, 0.0)
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &Hittable) -> Color {
        if depth <= 0 {
            Color::black()
        } else if let Some(record) = world.hit(ray, &Interval::new(0.001, math::INFINITY)) {
            if self.diffuse {
                let direction = Vec3D::random_on_hemisphere(&record.normal());
                self.ray_color(&Ray::new(record.p().clone(), direction.0), depth - 1, world) * 0.5
            } else {
                Color::from(math::normalize_to_01(&record.normal().0))
            }
        } else {
            let unit_direction = ray.direction.to_unit().0;
            let a = math::normalize_to_01(unit_direction.y);
            Color::white() * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
        }
    }

    fn viewport(&self) -> Viewport {
        Viewport::new(2.0, &self, &self.image)
    }

    fn pixel_sample_scale(&self) -> Real {
        1.0 / self.samples_per_pixel as Real
    }
}

pub struct CameraBuilder {
    center: Option<Point>,
    focal_length: Option<f64>,
    image: Option<Image>,
    samples_per_pixel: Option<u32>,
    antialiasing: Option<bool>,
    diffuse: Option<bool>,
    max_depth: Option<u32>,
    config: Config,
}

impl CameraBuilder {
    fn new(config: Config) -> CameraBuilder {
        CameraBuilder {
            center: None,
            focal_length: None,
            image: None,
            samples_per_pixel: None,
            antialiasing: None,
            diffuse: None,
            max_depth: None,
            config,
        }
    }

    pub fn center(&mut self, center: Point) -> &mut Self {
        self.center = Some(center);
        self
    }

    pub fn focal_length(&mut self, focal_length: f64) -> &mut Self {
        self.focal_length = Some(focal_length);
        self
    }

    pub fn image(&mut self, image: Image) -> &mut Self {
        self.image = Some(image);
        self
    }

    pub fn samples_per_pixel(&mut self, samples_per_pixel: u32) -> &mut Self {
        self.samples_per_pixel = Some(samples_per_pixel);
        self
    }

    pub fn antialiasing(&mut self, antialiasing: bool) -> &mut Self {
        self.antialiasing = Some(antialiasing);
        self
    }

    pub fn diffuse(&mut self, diffuse: bool) -> &mut Self {
        self.diffuse = Some(diffuse);
        self
    }

    pub fn max_depth(&mut self, max_depth: u32) -> &mut Self {
        self.max_depth = Some(max_depth);
        self
    }

    pub fn build(&self) -> Camera {
        let defaults = self.config.app().scene().camera().defaults();
        Camera {
            center: self.center.clone().unwrap_or({
                let center = defaults.center();
                Point::new(center[0], center[1], center[2])
            }),
            focal_length: self.focal_length.unwrap_or(defaults.focal_length()),
            image: self.image.clone().unwrap_or(Image::new(100, 1.0)),
            samples_per_pixel: self
                .samples_per_pixel
                .unwrap_or(defaults.samples_per_pixel()),
            antialiasing: self.antialiasing.unwrap_or(defaults.antialiasing()),
            diffuse: self.diffuse.unwrap_or(defaults.diffuse()),
            max_depth: self.max_depth.unwrap_or(defaults.max_depth()),
        }
    }
}

#[derive(Clone)]
pub struct Image {
    width: u32,

    // Ideal aspect ratio, not the actual one.
    aspect_ratio: f64,
}

impl Image {
    pub fn new(width: u32, aspect_ratio: f64) -> Image {
        Image {
            width,
            aspect_ratio,
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
        self.width as f64 / self.height() as f64
    }

    pub fn height(&self) -> u32 {
        let height = (self.width as f64 / self.aspect_ratio) as u32;
        if height < 1 { 1 } else { height }
    }
}

struct Viewport<'a> {
    height: f64,
    camera: &'a Camera,
    image: &'a Image,
}

impl<'a> Viewport<'a> {
    pub fn new(height: f64, camera: &'a Camera, image: &'a Image) -> Viewport<'a> {
        Viewport {
            height,
            camera,
            image,
        }
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn width(&self) -> f64 {
        self.height * self.image.actual_aspect_ratio()
    }

    pub fn left_to_right(&self) -> Vec3D {
        Vec3D::new(self.width(), 0.0, 0.0)
    }

    pub fn bottom_to_top(&self) -> Vec3D {
        Vec3D::new(0.0, -self.height(), 0.0)
    }

    pub fn pixel_delta_u(&self) -> Vec3D {
        self.left_to_right() / self.image.width as Real
    }

    pub fn pixel_delta_v(&self) -> Vec3D {
        self.bottom_to_top() / self.image.height() as Real
    }

    pub fn upper_left(&self) -> Point {
        &self.camera.center
            - Vec3D::new(0.0, 0.0, self.camera.focal_length)
            - self.left_to_right() / 2.0
            - self.bottom_to_top() / 2.0
    }

    pub fn pixel_00_loc(&self) -> Point {
        self.upper_left() + (self.pixel_delta_u() + self.pixel_delta_v()) * 0.5
    }
}
