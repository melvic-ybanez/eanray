pub mod camera;
mod color;
mod hit;
pub mod math;
mod ray;
pub(crate) mod shapes;

pub type Camera = camera::Camera;
pub type HittableList = hit::HittableList;
pub type Hittable = hit::Hittable;
pub type Color = color::Color;
pub type Ray = ray::Ray;
