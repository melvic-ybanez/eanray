use rand::Rng;
use std::ops::{Add, Mul};

pub(crate) mod interval;
pub(crate) mod macros;
pub(crate) mod vector;
pub(crate) mod ray;
mod matrix;
mod tuple;
pub(crate) mod point;

pub(crate) type Real = f64;

pub(crate) use point::Point;
pub(crate) use vector::Vec3D;
pub(crate) use vector::VecLike;

pub(crate) const INFINITY: Real = Real::INFINITY;
pub(crate) const PI: Real = std::f64::consts::PI;
pub(crate) const GAMMA: f32 = 2.2;
pub(crate) const EPSILON: f64 = 1e-8;

pub(crate) fn degrees_to_radians(degrees: Real) -> Real {
    degrees * PI / 180.0
}

pub(crate) fn normalize_to_01<A>(value: A) -> <A::Output as Mul<Real>>::Output
where
    A: Add<Real>,
    A::Output: Mul<Real>,
{
    (value + 1.0) * 0.5
}

pub(crate) fn random_range(min: Real, max: Real) -> Real {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub(crate) fn random_real() -> Real {
    random_range(0.0, 1.0)
}

pub(crate) fn random_int(min: i32, max: i32) -> i32 {
    random_range(min as Real, (max + 1) as Real) as i32
}

#[derive(PartialEq, Debug)]
pub(crate) enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub(crate) const fn from_usize(i: usize) -> Option<Axis> {
        match i {
            0 => Some(Axis::X),
            1 => Some(Axis::Y),
            2 => Some(Axis::Z),
            _ => None,
        }
    }

    pub(crate) const fn from_usize_unsafe(i: usize) -> Axis {
        Self::from_usize(i).unwrap()
    }
}

pub(crate) fn lerp<A>(start: A, end: A, a: Real) -> <A::Output as Add<A::Output>>::Output
where
    A: Mul<Real>,
    A::Output: Add<A::Output>,
{
    start * (1.0 - a) + end * a
}


pub(crate) fn near_zero(value: Real) -> bool {
   value < EPSILON
}

pub(crate) fn discriminant(a: Real, b: Real, c: Real) -> Real {
    b * b - 4.0 * a * c
}

pub(crate) fn root(a: Real, b: Real, sqrt_d: Real) -> Real {
    (-b + sqrt_d) / (2.0 * a)
}