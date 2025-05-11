use crate::core::color::Color;
use crate::core::hit::Hittable;
use crate::core::math::interval::Interval;
use crate::core::math::vector::{Point, Vec3D};
use crate::core::math::{self, Real};
use crate::core::ray::Ray;
use std::fs::File;
use std::io::{self, Write};

pub struct Camera {
    center: Point,
    pub focal_length: f64,
    pub image: Image,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            center: Point::zero(),
            focal_length: 1.0,
            image: Image::new(100, 1.0),
        }
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn render(&self, world: Hittable) -> io::Result<()> {
        let (viewport, ppm_file) = self.initialize()?;

        // output the PPM contents
        writeln!(
            &ppm_file,
            "P3\n{} {}\n255",
            self.image.width,
            self.image.height()
        )?;

        for j in 0..self.image.height() {
            println!("Scanlines remaining: {}", self.image.height() - j);
            for i in 0..self.image.width {
                let pixel_center = viewport.pixel_00_loc()
                    + (viewport.pixel_delta_u() * i as Real)
                    + (viewport.pixel_delta_v() * j as Real);
                let ray_direction = pixel_center - self.center();
                let ray = Ray::new(self.center().clone(), ray_direction);
                let pixel_color = self.ray_color(&ray, &world);
                pixel_color.write_to_file(&ppm_file)?
            }
        }

        println!("Done!");

        Ok(())
    }

    fn ray_color(&self, ray: &Ray, world: &Hittable) -> Color {
        if let Some(record) = world.hit(ray, &Interval::new(0.0, math::INFINITY)) {
            Color::from(math::normalize_to_01(&record.normal().0))
        } else {
            let unit_direction = ray.direction().to_unit().0;
            let a = math::normalize_to_01(unit_direction.y());
            Color::white() * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
        }
    }

    fn initialize(&self) -> io::Result<(Viewport, File)> {
        let ppm_file = File::create("output.ppm")?;
        let view_port = Viewport::new(2.0, &self, &self.image);
        Ok((view_port, ppm_file))
    }
}

pub struct Image {
    pub width: u32,

    // Ideal aspect ratio, not the actual one.
    pub aspect_ratio: f64,
}

impl Image {
    pub fn new(width: u32, aspect_ratio: f64) -> Image {
        Image {
            width,
            aspect_ratio,
        }
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
        self.camera.center()
            - Vec3D::new(0.0, 0.0, self.camera.focal_length)
            - self.left_to_right() / 2.0
            - self.bottom_to_top() / 2.0
    }

    pub fn pixel_00_loc(&self) -> Point {
        self.upper_left() + (self.pixel_delta_u() + self.pixel_delta_v()) * 0.5
    }
}
