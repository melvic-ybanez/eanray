use std::fmt::{Display, Formatter};
use rand::Rng;
use std::ops::{Add, Mul};

pub mod interval;
pub mod vector;
mod macros;

pub type Real = f64;

pub use vector::Point;
pub use vector::Vec3D;
pub use vector::VecLike;

pub const INFINITY: Real = Real::INFINITY;
pub const PI: Real = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: Real) -> Real {
    degrees * PI / 180.0
}

pub fn normalize_to_01<A>(value: A) -> <A::Output as Mul<Real>>::Output
where
    A: Add<Real>,
    A::Output: Mul<Real>,
{
    (value + 1.0) * 0.5
}

pub fn random_range(min: Real, max: Real) -> Real {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn random_real() -> Real {
    random_range(0.0, 1.0)
}

pub fn random_int(min: i32, max: i32) -> i32 {
    random_range(min as Real, (max + 1) as Real) as i32
}

#[derive(PartialEq, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const fn from_usize(i: usize) -> Option<Axis> {
        match i {
            0 => Some(Axis::X),
            1 => Some(Axis::Y),
            2 => Some(Axis::Z),
            _ => None,
        }
    }

    pub const fn from_usize_unsafe(i: usize) -> Axis {
        Self::from_usize(i).unwrap()
    }
}
