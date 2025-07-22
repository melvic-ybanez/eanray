use crate::core::math::vector::{Point, Vec3D};
use crate::core::math::Real;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ray<'a> {
    origin: Cow<'a, Point>,
    direction: Vec3D,
    time: Real,
}

impl<'a> Ray<'a> {
    pub fn from_cow_origin(origin: Cow<'a, Point>, direction: Vec3D) -> Ray<'a> {
        Self::from_cow_origin_timed(origin, direction, 0.0)
    }

    pub fn from_cow_origin_timed(origin: Cow<'a, Point>, direction: Vec3D, time: Real) -> Ray<'a> {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn new(origin: &'a Point, direction: Vec3D) -> Ray<'a> {
        Self::from_cow_origin(Cow::Borrowed(origin), direction)
    }

    pub fn new_timed(origin: &'a Point, direction: Vec3D, time: Real) -> Ray<'a> {
        Self::from_cow_origin_timed(Cow::Borrowed(origin), direction, time)
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

    pub fn time(&self) -> Real {
        self.time
    }
}
