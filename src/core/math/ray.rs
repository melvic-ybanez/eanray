use crate::core::math::vector::Vec3D;
use crate::core::math::{Point, Real};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Ray {
    origin: Point,
    direction: Vec3D,
    time: Real,
}

impl Ray {
    pub(crate) fn new(origin: Point, direction: Vec3D) -> Ray {
        Self::new_timed(origin, direction, 0.0)
    }

    pub(crate) fn new_timed(origin: Point, direction: Vec3D, time: Real) -> Ray {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub(crate) fn at(&self, t: Real) -> Point {
        &self.origin + &self.direction * t
    }

    pub(crate) fn origin(&self) -> &Point {
        &self.origin
    }

    pub(crate) fn direction(&self) -> &Vec3D {
        &self.direction
    }

    pub(crate) fn time(&self) -> Real {
        self.time
    }
}
