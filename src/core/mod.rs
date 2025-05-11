pub mod camera;
mod color;
mod hit;
mod ray;
pub mod math;
pub(crate) mod shapes;

pub type Camera = camera::Camera;
pub type HittableList = hit::HittableList;
pub type Hittable = hit::Hittable;
pub type Color = color::Color;
pub type Ray = ray::Ray;