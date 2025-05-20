use crate::core::math::Real;
use crate::core::math::vector::{Point, Vec3D};

#[derive(Clone)]
pub struct Ray<'a> {
    origin: &'a Point,
    direction: Vec3D,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point, direction: Vec3D) -> Ray<'a> {
        Ray { origin, direction }
    }

    pub fn at(&self, t: Real) -> Point {
        self.origin + &self.direction * t
    }
    
    pub fn origin(&self) -> &Point {
        self.origin
    }
    
    pub fn direction(&self) -> &Vec3D {
        &self.direction
    }
}
