use crate::core::camera::Image;
use crate::core::materials::refractive_index;
use crate::core::math::vector::Coordinates;
use crate::core::math::Real;
use crate::core::Camera as CoreCamera;
use crate::core::{self, math, shapes, Hittable, HittableList};
use crate::dsl::expr::{EvalResult, EvalResultF};
use crate::dsl::Expr;
use crate::settings::Config;
use serde::Deserialize;

type Vec3D = [Number; 3];
type Point = Vec3D;
type Color = Vec3D;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(Clone)]
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

    fn build(&self, config: &'static Config) -> EvalResultF<CoreCamera> {
        let builder_config = config.clone();
        let defaults = builder_config.app().scene().camera().defaults();
        let center = build_point(
            &self
                .center
                .clone()
                .unwrap_or(defaults.center().map(|comp| Number::Value(comp))),
        );

        center.map(|center| {
            CoreCamera::builder(config)
                .center(center)
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
        })
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
    radius: Number,
    material: Material,
}

impl Sphere {
    fn build(&self) -> EvalResultF<shapes::Sphere> {
        self.material.build().and_then(|mat| {
            self.radius.eval().and_then(|radius| {
                build_point(&self.center).map(|center| shapes::Sphere::new(center, radius, mat))
            })
        })
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
                build_color(albedo).map(|albedo| core::Material::new_lambertian(albedo))
            }
            Material::Metal(Metal { albedo, fuzz }) => {
                build_color(albedo).map(|albedo| core::Material::new_metal(albedo, *fuzz))
            }
            Material::Dielectric(Dielectric { refraction_index }) => {
                let index_result = match refraction_index {
                    RefractionIndex::Custom(Number::Value(value)) => Ok(*value),
                    RefractionIndex::Custom(Number::Expr(expr)) => Expr::eval_str(expr),
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
    Custom(Number),
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
#[derive(Clone)]
pub enum Number {
    Value(Real),
    Expr(String),
}

impl Number {
    fn eval(&self) -> EvalResult {
        match self {
            Number::Value(real) => Ok(*real),
            Number::Expr(expr) => Expr::eval_str(expr),
        }
    }
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
        camera.and_then(|camera| {
            objects.map(|shapes| {
                let objects = HittableList::from_vec(shapes);
                (camera, Hittable::List(objects))
            })
        })
    }
}

fn build_point(point: &Point) -> EvalResultF<math::Point> {
    let p: EvalResultF<Vec<Real>> = point.iter().map(|x| x.eval()).collect();

    p.map(|coords| math::Point::new(coords[0], coords[1], coords[2]))
}

fn build_color(color: &Color) -> EvalResultF<core::Color> {
    let p: EvalResultF<math::Point> = build_point(color);

    p.map(|point| core::Color::new(point.x(), point.y(), point.z()))
}
