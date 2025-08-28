use std::ops::{Index, IndexMut};
use crate::core::math::Real;

pub(crate) const SIZE: usize = 4;

pub(crate) type Elems = [Real; SIZE];

pub(crate) struct Tuple {
    elems: Elems
}

impl Tuple {
    pub(crate) fn from_elems(elems: Elems) -> Self {
        Self { elems }
    }

    pub(crate) fn elems(&self) -> &Elems {
        &self.elems
    }
}

impl Index<usize> for Tuple {
    type Output = Real;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl IndexMut<usize> for Tuple {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elems[index]
    }
}