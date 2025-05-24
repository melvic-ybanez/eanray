use crate::core::Camera as CoreCamera;
use crate::core::camera::Image;
use crate::core::math::Real;
use crate::core::{self, Hittable, HittableList, math, shapes};
use crate::settings::Config;
use serde::Deserialize;

type Vec3D = [Real; 3];
type Point = Vec3D;
type Color = Vec3D;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Camera {
    center: Option<Point>,
    focal_length: Option<Real>,
    aspect_ratio: [Real; 2],
    image_width: u32,
    samples_per_pixel: Option<u32>,
    antialiasing: Option<bool>,
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
            .max_depth(self.max_depth.unwrap_or(defaults.max_depth()))
            .build()
    }
}

#[derive(Deserialize)]
pub struct Object {
    description: Option<String>,
    #[serde(flatten)]
    shape: Shape,
}

impl Object {
    fn build(&self) -> Hittable {
        self.shape.build()
    }
}

#[derive(Deserialize)]
#[serde(tag = "shape")]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    fn build(&self) -> Hittable {
        match self {
            Shape::Sphere(sphere) => Hittable::Sphere(sphere.build()),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sphere {
    center: Point,
    radius: Real,
    material: Material,
}

impl Sphere {
    fn build(&self) -> shapes::Sphere {
        shapes::Sphere::new(build_point(self.center), self.radius, self.material.build())
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric)
}

impl Material {
    pub fn build(&self) -> core::Material {
        match *self {
            Material::Lambertian(Lambertian { albedo }) => core::Material::Lambertian {
                albedo: build_color(albedo),
            },
            Material::Metal(Metal { albedo, fuzz }) => {
                core::Material::new_metal(build_color(albedo), fuzz)
            }
            Material::Dielectric(Dielectric { refraction_index }) => {
                core::Material::Dielectric { refraction_index }
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lambertian {
    albedo: Color,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metal {
    albedo: Color,
    fuzz: Real,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dielectric {
    refraction_index: Real
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
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

fn build_color(p: Color) -> core::Color {
    core::Color::new(p[0], p[1], p[2])
}
