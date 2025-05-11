use crate::math::Real;
use crate::math::vector::{Point, Vec3D};

#[derive(Clone)]
pub struct Ray {
    origin: Point,
    direction: Vec3D,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3D) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3D {
        &self.direction
    }

    pub fn at(&self, t: Real) -> Point {
        &self.origin + &self.direction * t
    }
}
