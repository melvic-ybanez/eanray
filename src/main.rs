use crate::camera::Camera;
use crate::hit::{Hittable, HittableList};
use crate::math::vector::Point;
use crate::shapes::sphere::Sphere;
use std::io;

mod camera;
mod color;
mod hit;
mod math;
mod ray;
mod shapes;

fn main() -> io::Result<()> {
    let world = Hittable::List(HittableList::from_vec(vec![
        Hittable::Sphere(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)),
        Hittable::Sphere(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)),
    ]));

    let mut camera = Camera::new();
    camera.image.aspect_ratio = 16.0 / 9.0;
    camera.image.width = 400;

    camera.render(world)
}
