use crate::core::math::tuple::{Elems, Tuple, SIZE};
use std::default::Default;
use std::ops::{Index, IndexMut, Mul};

type Table = [Row; SIZE];
type Row = Elems;

pub(crate) struct Matrix {
    elems: Table,
}

impl Matrix {
    pub(crate) fn new(rows: Table) -> Self {
        Self { elems: rows }
    }

    pub(crate) fn identify() -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self {
            elems: Table::default(),
        }
    }
}

impl PartialEq<Self> for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.elems == other.elems
    }
}

impl Index<usize> for Matrix {
    type Output = Row;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elems[index]
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Self::Output {
        let mut result: Matrix = Matrix::default();
        for row in 0..SIZE {
            for col in 0..SIZE {
                for i in 0..SIZE {
                    result[row][col] += self[row][i] * other[i][col];
                }
            }
        }
        result
    }
}

impl Mul<&Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, tuple: &Tuple) -> Self::Output {
        let mut result = Elems::default();
        for row in 0..SIZE {
            for i in 0..SIZE {
                result[row] += self[row][i] * tuple[i];
            }
        }
        Tuple::from_elems(result)
    }
}
