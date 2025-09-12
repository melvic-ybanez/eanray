pub(crate) mod matrix_4x4;

#[cfg(test)]
mod tests;

use crate::common::macros::impl_index;
use crate::core::math::tuple::{Elems, Tuple4};
use crate::core::math::Real;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut, Mul};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Matrix {
    elems: Vec<Real>,
    order: usize,
    determinant: Option<Real>,
}

impl Matrix {
    pub(crate) fn from_vec(elems: Vec<Real>) -> Self {
        let order = elems.len().isqrt();
        let mut this = Self {
            elems,
            order,
            determinant: None,
        };
        this
    }

    pub(crate) fn fill(order: usize, value: Real) -> Self {
        Self::from_vec(vec![value; order * order])
    }

    pub(crate) fn fill_default(order: usize) -> Self {
        Self::fill(order, Real::default())
    }

    pub(crate) fn transpose(&self) -> Self {
        let mut transposition: Matrix = self.clone();
        self.traverse(|row, col| {
            transposition[(row, col)] = self[(col, row)];
        });
        transposition
    }

    /// Yields a new smaller matrix that does not contain the
    /// row `row` and column `col`.
    fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut table = vec![];

        self.traverse(|r, c| {
            if r != row && c != col {
                table.push(self[(r, c)]);
            }
        });

        Self::from_vec(table)
    }

    fn determinant(&mut self) -> Real {
        if let Some(determinant) = self.determinant {
            determinant
        } else {
            let mut determinant = 0.0;
            if self.order == 2 {
                determinant = self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)];
            } else {
                for col in 0..self.order {
                    determinant += self[(0, col)] * self.cofactor(0, col);
                }
            }
            self.determinant = Some(determinant);
            determinant
        }
    }

    /// The minor at (`row`, `col`) of a matrix, defined as
    /// the determinant of the submatrix at (`row`, `col`).
    fn minor(&self, row: usize, col: usize) -> Real {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> Real {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 { minor } else { -minor }
    }

    fn is_invertible(&mut self) -> bool {
        Self::check_invertibility(self.determinant())
    }

    fn check_invertibility(determinant: Real) -> bool {
        determinant != 0.0
    }

    pub(crate) fn inverse(&mut self) -> Option<Matrix> {
        let determinant = self.determinant();

        if Self::check_invertibility(determinant) {
            let mut matrix = Self::fill_default(self.order);

            self.traverse(|row, col| {
                let c = self.cofactor(row, col);

                // we are reversing `row` and `col` to transpose the matrix
                matrix[(col, row)] = c / determinant;
            });

            Some(matrix)
        } else {
            None
        }
    }

    pub(crate) fn inverse_unsafe(&mut self) -> Matrix {
        self.inverse().unwrap()
    }

    fn traverse<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize) -> (),
    {
        for row in 0..self.order {
            for col in 0..self.order {
                f(row, col);
            }
        }
    }
}

fn fold_index(index: (usize, usize), size: usize) -> usize {
    let (row, col) = index;
    row * size + col
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order && {
            for i in 0..self.order * self.order {
                if (self[i] - other[i]).abs() >= 1e-5 {
                    return false;
                }
            }
            true
        }
    }
}

impl_index!(usize, Matrix, Real, elems);

impl Index<(usize, usize)> for Matrix {
    type Output = Real;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self[fold_index(index, self.order)]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let order = self.order;
        &mut self[fold_index(index, order)]
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Self::Output {
        let mut result: Matrix = Matrix::fill_default(other.order);
        other.traverse(|row, col| {
            // this is effectively a series of dot products
            for i in 0..other.order {
                result[(row, col)] += self[(row, i)] * other[(i, col)];
            }
        });
        result
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&Tuple4> for &Matrix {
    type Output = Tuple4;

    fn mul(self, tuple: &Tuple4) -> Self::Output {
        let mut result = Elems::default();
        for row in 0..self.order {
            // like dot product, but for a single column
            for i in 0..self.order {
                result[row] += self[(row, i)] * tuple[i];
            }
        }
        Tuple4::from_vec(result)
    }
}

impl Mul<Tuple4> for &Matrix {
    type Output = Tuple4;

    fn mul(self, rhs: Tuple4) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Tuple4> for Matrix {
    type Output = Tuple4;

    fn mul(self, rhs: Tuple4) -> Self::Output {
        &self * &rhs
    }
}
