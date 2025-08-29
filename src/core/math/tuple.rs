use std::ops::{Index, IndexMut};
use crate::core::math::Real;

pub(crate) type Elems = [Real; 4];

pub(crate) struct Tuple4 {
    elems: Elems
}

impl Tuple4 {
    pub(crate) fn from_elems(elems: Elems) -> Self {
        Self { elems }
    }

    pub(crate) fn elems(&self) -> &Elems {
        &self.elems
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