use crate::core::math::Real;
use crate::core::math::vector::{Point, Vec3D};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ray {
    origin: Point,
    direction: Vec3D,
    time: Real,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3D) -> Ray {
        Self::new_timed(origin, direction, 0.0)
    }

    pub fn new_timed(origin: Point, direction: Vec3D, time: Real) -> Ray {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: Real) -> Point {
        &self.origin + &self.direction * t
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3D {
        &self.direction
    }

    pub fn time(&self) -> Real {
        self.time
    }
}
