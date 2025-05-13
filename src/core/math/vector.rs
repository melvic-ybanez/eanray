use crate::core::math::Real;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone)]
pub struct VecKind;

#[derive(Clone)]
pub struct PointKind;

pub type Vec3D = VecLike<VecKind>;
pub type Point = VecLike<PointKind>;

#[derive(Clone)]
pub struct VecLike<Kind> {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    _kind: PhantomData<Kind>,
}

pub struct UnitVec3D(pub Vec3D);

impl<K> VecLike<K> {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self {
            x,
            y,
            z,
            _kind: PhantomData,
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Vec3D {
    pub fn length(&self) -> Real {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Real {
        self.dot(self)
    }

    pub fn dot(&self, rhs: &Vec3D) -> Real {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3D) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn to_unit(&self) -> UnitVec3D {
        UnitVec3D(self / self.length())
    }
}

pub trait CanAdd {}

impl CanAdd for VecKind {}

impl<V> Neg for VecLike<V> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<V: CanAdd> Add<VecLike<V>> for &VecLike<V> {
    type Output = VecLike<V>;

    fn add(self, rhs: VecLike<V>) -> Self::Output {
        VecLike::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<V: CanAdd> Add<VecLike<V>> for VecLike<V> {
    type Output = VecLike<V>;

    fn add(self, rhs: VecLike<V>) -> Self::Output {
        &self + rhs
    }
}

impl<V: CanAdd> Add<&VecLike<V>> for VecLike<V> {
    type Output = VecLike<V>;
    
    fn add(self, rhs: &VecLike<V>) -> Self::Output {
        rhs + self   
    }   
}

impl<V: CanAdd> Add<Real> for VecLike<V> {
    type Output = VecLike<V>;

    fn add(self, rhs: Real) -> Self::Output {
        &self + rhs
    }
}

impl<V: CanAdd> Add<Real> for &VecLike<V> {
    type Output = VecLike<V>;

    fn add(self, rhs: Real) -> Self::Output {
        self + VecLike::new(rhs, rhs, rhs)
    }
}

impl Add<Vec3D> for &Point {
    type Output = Point;

    fn add(self, rhs: Vec3D) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Vec3D> for Point {
    type Output = Point;

    fn add(self, rhs: Vec3D) -> Self::Output {
        &self + rhs
    }
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

impl Sub<Vec3D> for &Point {
    type Output = Point;

    fn sub(self, rhs: Vec3D) -> Self::Output {
        self + -rhs
    }
}

impl Sub<Vec3D> for Point {
    type Output = Point;

    fn sub(self, rhs: Vec3D) -> Self::Output {
        &self - rhs
    }
}

impl<K> Mul<VecLike<K>> for &VecLike<K> {
    type Output = VecLike<K>;

    fn mul(self, rhs: VecLike<K>) -> Self::Output {
        VecLike::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<K> Mul<Real> for &VecLike<K> {
    type Output = VecLike<K>;

    fn mul(self, t: Real) -> Self::Output {
        self * VecLike::new(t, t, t)
    }
}

impl<K> Mul<Real> for VecLike<K> {
    type Output = VecLike<K>;

    fn mul(self, t: Real) -> Self::Output {
        &self * VecLike::new(t, t, t)
    }
}

impl<K> Div<Real> for &VecLike<K> {
    type Output = VecLike<K>;

    fn div(self, rhs: Real) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl<K> Div<Real> for VecLike<K> {
    type Output = VecLike<K>;

    fn div(self, rhs: Real) -> Self::Output {
        &self / rhs
    }
}

impl<K> fmt::Display for VecLike<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
