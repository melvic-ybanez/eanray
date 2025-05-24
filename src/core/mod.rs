pub mod camera;
pub mod color;
mod hit;
pub mod math;
mod ray;
pub mod shapes;
pub mod materials;

pub use camera::Camera;
pub use hit::HittableList;
pub use hit::Hittable;
pub use color::Color;
pub use ray::Ray;
pub use materials::Material;
