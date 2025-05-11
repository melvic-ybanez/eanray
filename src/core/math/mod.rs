use std::ops::{Add, Mul};

pub mod vector;
pub mod interval;

pub type Real = f64;

pub type Vec3D = vector::Vec3D;
pub type Point = vector::Point;

pub const INFINITY: Real = Real::INFINITY;
pub const PI: Real = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: Real) -> Real {
    degrees * PI / 180.0
}

pub fn normalize_to_01<A>(value: A) -> <A::Output as Mul<Real>>::Output
where
    A: Add<Real>,
    A::Output: Mul<Real>,{
    (value + 1.0) * 0.5
}