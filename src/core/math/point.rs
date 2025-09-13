use crate::core::math::macros::define_tuple_conversion;
use crate::core::math::matrix::Matrix;
use crate::core::math::{Vec3D, VecLike};
use std::ops::Sub;

#[derive(Clone, Debug)]
pub(crate) struct PointKind;

pub(crate) type Point = VecLike<PointKind>;

impl Point {
    define_tuple_conversion!();
}

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
