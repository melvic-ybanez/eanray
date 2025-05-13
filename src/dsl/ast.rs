use crate::core;
use crate::core::math::Real;
use crate::core::{Hittable, HittableList, math, shapes};
use serde::Deserialize;

type Vec3D = [Real; 3];
type Point = Vec3D;
type Color = Vec3D;

#[derive(Deserialize)]
pub struct Camera {
    #[serde(default = "Camera::default_center")]
    center: Point,

    #[serde(default = "Camera::default_focal_length")]
    focal_length: f64,

    aspect_ratio: [Real; 2],
    image_width: u32,

    #[serde(default = "Camera::default_samples_per_pixel")]
    samples_per_pixel: u32,
}

impl Camera {
    fn default_center() -> Point {
        let center = core::Camera::default_center();
        [center.x, center.y, center.z]
    }
    
    fn default_focal_length() -> Real {
        core::Camera::DEFAULT_FOCAL_LENGTH
    }
    
    fn default_samples_per_pixel() -> u32 {
        core::Camera::DEFAULT_SAMPLES_PER_PIXEL
    }

    fn ideal_aspect_ratio(&self) -> Real {
        self.aspect_ratio[0] / self.aspect_ratio[1]
    }

    fn build(&self) -> core::Camera {
        let mut camera = core::Camera::new();
        camera.image.aspect_ratio = self.ideal_aspect_ratio();
        camera.image.width = self.image_width;
        camera
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Object {
    Sphere { sphere: Sphere },
}

impl Object {
    fn build(&self) -> Hittable {
        match self {
            Object::Sphere { sphere } => Hittable::Sphere(sphere.build()),
        }
    }
}

#[derive(Deserialize)]
pub struct Sphere {
    center: Point,
    radius: Real,
}

impl Sphere {
    fn build(&self) -> shapes::Sphere {
        shapes::Sphere::new(build_point(self.center), self.radius)
    }
}

#[derive(Deserialize)]
pub struct Scene {
    camera: Camera,
    objects: Vec<Object>,
}

impl Scene {
    pub fn build(&self) -> (core::Camera, Hittable) {
        let camera = self.camera.build();
        let objects = HittableList::from_vec(self.objects.iter().map(|o| o.build()).collect());
        (camera, Hittable::List(objects))
    }
}

fn build_point(p: Point) -> math::Point {
    math::Point::new(p[0], p[1], p[2])
}
