use crate::core::camera::Image;
use crate::core::materials::refractive_index;
use crate::core::math::Real;
use crate::core::Camera as CoreCamera;
use crate::core::{self, math, shapes, Hittable, HittableList};
use crate::dsl::expr::{EvalResult, EvalResultF};
use crate::dsl::Expr;
use crate::settings;
use crate::settings::Config;
use serde::Deserialize;

type Vec3D = [Number; 3];
type Point = Vec3D;
type Color = Vec3D;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Camera {
    aspect_ratio: [Real; 2],
    image_width: u32,
    samples_per_pixel: Option<u32>,
    antialiasing: Option<bool>,
    max_depth: Option<u32>,
    field_of_view: Option<Real>,
    look_from: Option<Point>,
    look_at: Option<Point>,
    defocus_angle: Option<Number>,
    focus_distance: Option<Number>,
    vup: Option<Vec3D>,
}

impl Camera {
    fn ideal_aspect_ratio(&self) -> Real {
        self.aspect_ratio[0] / self.aspect_ratio[1]
    }

    fn build(&self, config: &'static Config) -> EvalResultF<CoreCamera> {
        let builder_config = config.clone();
        let defaults = builder_config.app().scene().camera().defaults();

        fn build_vec_like_with_default<K>(
            vec_like: &Option<Vec3D>,
            default: settings::Point,
        ) -> EvalResultF<math::VecLike<K>> {
            build_vec_like(
                &vec_like
                    .clone()
                    .unwrap_or(default.map(|comp| Number::Value(comp))),
            )
        }

        let look_from = build_vec_like_with_default(&self.look_from, defaults.look_from())?;
        let look_at = build_vec_like_with_default(&self.look_at, defaults.look_at())?;
        let vup = build_vec_like_with_default(&self.vup, defaults.vup())?;
        let defocus_angle = self
            .defocus_angle
            .clone()
            .unwrap_or(Number::Value(defaults.defocus_angle()))
            .eval()?;
        let focus_distance = self
            .focus_distance
            .clone()
            .unwrap_or(Number::Value(defaults.focus_distance()))
            .eval()?;

        Ok(CoreCamera::builder(config)
            .image(Image::new(self.image_width, self.ideal_aspect_ratio()))
            .antialiasing(self.antialiasing.unwrap_or(defaults.antialiasing()))
            .samples_per_pixel(
                self.samples_per_pixel
                    .unwrap_or(defaults.samples_per_pixel()),
            )
            .max_depth(self.max_depth.unwrap_or(defaults.max_depth()))
            .field_of_view(self.field_of_view.unwrap_or(defaults.field_of_view()))
            .look_from(look_from)
            .look_at(look_at)
            .defocus_angle(defocus_angle)
            .focus_distance(focus_distance)
            .vup(vup)
            .build())
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
        let mat = self.material.build()?;
        let radius = self.radius.eval()?;

        build_vec_like(&self.center).map(|center| shapes::Sphere::new(center, radius, mat))
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
                build_vec_like(albedo).map(|albedo| core::Material::new_lambertian(albedo))
            }
            Material::Metal(Metal { albedo, fuzz }) => {
                build_vec_like(albedo).map(|albedo| core::Material::new_metal(albedo, *fuzz))
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

fn build_vec_like<K>(point: &Point) -> EvalResultF<math::VecLike<K>> {
    let p: EvalResultF<Vec<Real>> = point.iter().map(|x| x.eval()).collect();

    p.map(|coords| math::VecLike::new(coords[0], coords[1], coords[2]))
}
