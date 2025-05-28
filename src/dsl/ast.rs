use crate::core::Camera as CoreCamera;
use crate::core::camera::Image;
use crate::core::materials::refractive_index;
use crate::core::math::Real;
use crate::core::{self, Hittable, HittableList, math, shapes};
use crate::dsl;
use crate::dsl::expr::EvalResultF;
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
    field_of_view: Option<Real>,
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
            .field_of_view(self.field_of_view.unwrap_or(defaults.field_of_view()))
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
    fn build(&self) -> EvalResultF<Hittable> {
        self.shape.build()
    }
}

#[derive(Deserialize)]
#[serde(tag = "shape")]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    fn build(&self) -> EvalResultF<Hittable> {
        match self {
            Shape::Sphere(sphere) => sphere.build().map(Hittable::Sphere),
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
    fn build(&self) -> EvalResultF<shapes::Sphere> {
        self.material
            .build()
            .map(|mat| shapes::Sphere::new(build_point(self.center), self.radius, mat))
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn build(&self) -> EvalResultF<core::Material> {
        match self {
            Material::Lambertian(Lambertian { albedo }) => {
                Ok(core::Material::new_lambertian(build_color(albedo.clone())))
            }
            Material::Metal(Metal { albedo, fuzz }) => Ok(core::Material::new_metal(
                build_color(albedo.clone()),
                *fuzz,
            )),
            Material::Dielectric(Dielectric { refraction_index }) => {
                let index_result = match refraction_index {
                    RefractionIndex::Custom(Expr::Num(value)) => Ok(*value),
                    RefractionIndex::Custom(Expr::Complex(expr)) => dsl::expr::eval(expr),
                    RefractionIndex::Label(RefractionIndexLabel::Glass) => {
                        Ok(refractive_index::GLASS)
                    }
                    RefractionIndex::Label(RefractionIndexLabel::Air) => Ok(refractive_index::AIR),
                    RefractionIndex::Label(RefractionIndexLabel::Vacuum) => {
                        Ok(refractive_index::VACUUM)
                    }
                    RefractionIndex::Label(RefractionIndexLabel::Water) => {
                        Ok(refractive_index::WATER)
                    }
                    RefractionIndex::Label(RefractionIndexLabel::Diamond) => {
                        Ok(refractive_index::DIAMOND)
                    }
                };
                index_result.map(core::Material::new_dielectric)
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
    refraction_index: RefractionIndex,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RefractionIndex {
    Label(RefractionIndexLabel),
    Custom(Expr),
}

#[derive(Deserialize)]
pub enum RefractionIndexLabel {
    Glass,
    Air,
    Vacuum,
    Water,
    Diamond,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Expr {
    Num(Real),
    Complex(String),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    camera: Camera,
    objects: Vec<Object>,
}

impl Scene {
    pub fn build(&self, config: &'static Config) -> EvalResultF<(CoreCamera, Hittable)> {
        let camera = self.camera.build(config);
        let objects: EvalResultF<Vec<Hittable>> = self.objects.iter().map(Object::build).collect();
        objects.map(|shapes| {
            let objects = HittableList::from_vec(shapes);
            (camera, Hittable::List(objects))
        })
    }
}

fn build_point(p: Point) -> math::Point {
    math::Point::new(p[0], p[1], p[2])
}

fn build_color(p: Color) -> core::Color {
    core::Color::new(p[0], p[1], p[2])
}
