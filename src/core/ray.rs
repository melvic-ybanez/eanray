use std::borrow::Cow;
use crate::core::math::vector::{Point, Vec3D};
use crate::core::math::Real;

#[derive(Clone)]
pub struct Ray<'a> {
    origin: Cow<'a, Point>,
    direction: Vec3D,
}

impl<'a> Ray<'a> {
    pub fn new(origin: Cow<'a, Point>, direction: Vec3D) -> Ray<'a> {
        Ray { origin, direction }
    }
    
    pub fn from_ref_origin(origin: &'a Point, direction: Vec3D) -> Ray<'a> {
        Self::new(Cow::Borrowed(origin), direction)
    }

    pub fn at(&self, t: Real) -> Point {
        self.origin.as_ref() + &self.direction * t
    }

    pub fn origin(&self) -> &Point {
        self.origin.as_ref()
    }

    pub fn direction(&self) -> &Vec3D {
        &self.direction
    }
}
