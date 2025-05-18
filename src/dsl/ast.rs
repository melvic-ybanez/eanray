use crate::core::Camera as CoreCamera;
use crate::core::camera::Image;
use crate::core::math::Real;
use crate::core::{Hittable, HittableList, math, shapes};
use crate::settings::Config;
use serde::Deserialize;

type Vec3D = [Real; 3];
type Point = Vec3D;
type Color = Vec3D;

#[derive(Deserialize)]
pub struct Camera {
    center: Option<Point>,
    focal_length: Option<Real>,
    aspect_ratio: [Real; 2],
    image_width: u32,
    samples_per_pixel: Option<u32>,
    antialiasing: Option<bool>,
    diffuse: Option<bool>,
    max_depth: Option<u32>,
}

impl Camera {
    fn ideal_aspect_ratio(&self) -> Real {
        self.aspect_ratio[0] / self.aspect_ratio[1]
    }

    fn build(&self, config: &'static Config) -> CoreCamera {
        let builder_config = config.clone();
        let defaults = builder_config.app().scene().camera().defaults();

        CoreCamera::builder(config)
            .center(build_point(self.center.unwrap_or(defaults.center())))
            .focal_length(self.focal_length.unwrap_or(defaults.focal_length()))
            .image(Image::new(self.image_width, self.ideal_aspect_ratio()))
            .antialiasing(self.antialiasing.unwrap_or(defaults.antialiasing()))
            .samples_per_pixel(
                self.samples_per_pixel
                    .unwrap_or(defaults.samples_per_pixel()),
            )
            .diffuse(self.diffuse.unwrap_or(defaults.diffuse()))
            .max_depth(self.max_depth.unwrap_or(defaults.max_depth()))
            .build()
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
    pub fn build(&self, config: &'static Config) -> (CoreCamera, Hittable) {
        let camera = self.camera.build(config);
        let objects = HittableList::from_vec(self.objects.iter().map(|o| o.build()).collect());
        (camera, Hittable::List(objects))
    }
}

fn build_point(p: Point) -> math::Point {
    math::Point::new(p[0], p[1], p[2])
}
