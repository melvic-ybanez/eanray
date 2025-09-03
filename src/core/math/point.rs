use std::ops::Sub;
use serde::{Deserialize, Serialize};
use crate::core::math::{Vec3D, VecLike};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct PointKind;

pub(crate) type Point = VecLike<PointKind>;

impl Sub for &Point {
    type Output = Vec3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Point> for &Point {
    type Output = Vec3D;

    fn sub(self, rhs: Point) -> Self::Output {
        self - &rhs
    }
}

impl Sub<&Point> for Point {
    type Output = Vec3D;

    fn sub(self, rhs: &Point) -> Self::Output {
        &self - rhs
    }
}

impl Sub<Point> for Point {
    type Output = Vec3D;

    fn sub(self, rhs: Point) -> Self::Output {
        &self - rhs
    }
}