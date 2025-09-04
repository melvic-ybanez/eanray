use crate::core::math::{Point, Real, Vec3D, VecLike};
use std::ops::{Index, IndexMut};

pub(crate) type Elems = [Real; 4];

pub(crate) struct Tuple4 {
    elems: Elems,
}

impl Tuple4 {
    pub(crate) fn new(x: Real, y: Real, z: Real, w: Real) -> Self {
        Self {
            elems: [x, y, z, w],
        }
    }

    pub(crate) fn from_elems(elems: Elems) -> Self {
        Self { elems }
    }

    pub(crate) fn elems(&self) -> &Elems {
        &self.elems
    }

    pub(crate) fn x(&self) -> Real {
        self.elems[0]
    }

    pub(crate) fn y(&self) -> Real {
        self.elems[1]
    }

    pub(crate) fn z(&self) -> Real {
        self.elems[2]
    }

    pub(crate) fn w(&self) -> Real {
        self.elems[3]
    }
}

impl Index<usize> for Tuple4 {
    type Output = Real;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl IndexMut<usize> for Tuple4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elems[index]
    }
}

impl From<&Point> for Tuple4 {
    fn from(point: &Point) -> Self {
        Self::new(point.x, point.y, point.z, 1.0)
    }
}

impl From<&Vec3D> for Tuple4 {
    fn from(vec: &Vec3D) -> Self {
        Self::new(vec.x, vec.y, vec.z, 0.0)
    }
}

impl<K> From<Tuple4> for VecLike<K> {
    fn from(tuple: Tuple4) -> Self {
        VecLike::new(tuple.x(), tuple.y(), tuple.z())
    }
}