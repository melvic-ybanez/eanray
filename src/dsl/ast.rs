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

    #[serde(default = "Camera::default_focal_point")]
    focal_point: f64,

    aspect_ratio: [Real; 2],
    image_width: u32,
}

impl Camera {
    fn default_center() -> Point {
        [0.0, 0.0, 0.0]
    }

    fn default_focal_point() -> f64 {
        1.0
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

fn build_vec3d(v: Vec3D) -> math::Vec3D {
    math::Vec3D::new(v[0], v[1], v[2])
}

fn build_point(p: Point) -> math::Point {
    math::Point::new(p[0], p[1], p[2])
}

fn build_color(v: Color) -> core::Color {
    core::Color::new(v[0], v[1], v[2])
}
