use crate::common::macros::impl_index;
use crate::core::math::tuple::{Elems, Tuple4};
use crate::core::math::Real;
use std::default::Default;
use std::ops::{Index, IndexMut, Mul};

#[derive(Clone, Debug)]
pub(crate) struct Matrix {
    elems: Vec<Real>,
    order: usize,
    determinant: Real,
}

impl Matrix {
    pub(crate) fn from_vec(elems: Vec<Real>) -> Self {
        let order = elems.len().isqrt();
        let mut this = Self {
            elems,
            order,
            determinant: 0.0,
        };
        this.compute_determinant();
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

    fn compute_determinant(&mut self) {
        if self.order == 2 {
            self.determinant = self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
        } else {
            for col in 0..self.order {
                self.determinant += self[(0, col)] * self.cofactor(0, col);
            }
        }
    }

    /// The minor at (`row`, `col`) of a matrix, defined as
    /// the determinant of the submatrix at (`row`, `col`).
    fn minor(&self, row: usize, col: usize) -> Real {
        self.submatrix(row, col).determinant
    }

    fn cofactor(&self, row: usize, col: usize) -> Real {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 { minor } else { -minor }
    }

    fn is_invertible(&self) -> bool {
        self.determinant != 0.0
    }

    fn inverse(&self) -> Option<Matrix> {
        if self.is_invertible() {
            let mut matrix = Self::fill_default(self.order);

            self.traverse(|row, col| {
                let c = self.cofactor(row, col);

                // we are reversing `row` and `col` to transpose the matrix
                matrix[(col, row)] = c / self.determinant;
            });

            Some(matrix)
        } else {
            None
        }
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

mod matrix_4x4 {
    use super::*;

    #[rustfmt::skip]
    pub(crate) fn identity() -> Matrix {
        from_array([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub(crate) fn fill(value: Real) -> Matrix {
        Matrix::fill(4, value)
    }

    pub(crate) fn fill_default() -> Matrix {
        Matrix::fill_default(4)
    }

    pub(crate) fn from_array(elems: [Real; 16]) -> Matrix {
        Matrix::from_vec(elems.to_vec())
    }

    pub(crate) fn from_2df(elems: [[Real; 4]; 4]) -> Matrix {
        from_array(elems.concat().try_into().unwrap())
    }

    pub(crate) fn from_2di(elems: [[u32; 4]; 4]) -> Matrix {
        from_2df(elems.map(|row| row.map(|f| f as Real)))
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
        Tuple4::from_elems(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn test_submatrices() {
        let matrix = matrix_4x4::from_2di([
            [1, 2, 3, 4],
            [0, 5, 6, 7],
            [8, 9, 1, 2],
            [3, 4, 5, 6]
        ]);

        let submatrix = Matrix::from_vec(vec![
            1.0, 2.0, 4.0,
            8.0, 9.0, 2.0,
            3.0, 4.0, 6.0
        ]);

        assert_eq!(matrix.submatrix(1, 2), submatrix);

        let matrix = matrix_4x4::from_2di([
            [4, 1, 0, 2],
            [7, 5, 9, 3],
            [6, 8, 2, 1],
            [5, 0, 3, 4]
        ]);

        let submatrix = Matrix::from_vec(vec![
            5.0, 9.0, 3.0,
            8.0, 2.0, 1.0,
            0.0, 3.0, 4.0
        ]);

        assert_eq!(matrix.submatrix(0, 0), submatrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_determinants() {
        let matrix = matrix_4x4::from_2di([
            [1, 2, 3, 4],
            [2, 0, 1, 5],
            [3, 1, 2, 6],
            [4, 2, 1, 0]
        ]);
        assert_eq!(matrix.determinant, -3.0);

        let matrix = matrix_4x4::from_2di([
            [2, 1, 0, 3],
            [4, 5, 6, 1],
            [7, 0, 8, 2],
            [1, 2, 3, 4]
        ]);
        assert_eq!(matrix.determinant, 450.0);

        let matrix = matrix_4x4::from_2di([
            [0, 2, 1, 3],
            [4, 1, 5, 2],
            [7, 0, 6, 1],
            [3, 2, 4, 0]
        ]);
        assert_eq!(matrix.determinant, -51.0);
    }

    #[rustfmt::skip]
    #[test]
    fn test_minors_and_cofactors() {
        let matrix = matrix_4x4::from_2di([
            [1, 2, 3, 4],
            [0, 5, 6, 7],
            [8, 9, 1, 2],
            [3, 4, 5, 6]
        ]);
        assert_eq!(matrix.minor(0, 0), -9.0);
        assert_eq!(matrix.cofactor(0, 0), -9.0);

        let matrix = matrix_4x4::from_2di([
            [2, 3, 4, 1],
            [0, 5, 6, 7],
            [8, 1, 2, 3],
            [4, 5, 6, 7]
        ]);
        assert_eq!(matrix.minor(1, 2), -112.0);
        assert_eq!(matrix.cofactor(1, 2), 112.0);
    }

    #[rustfmt::skip]
    #[test]
    fn test_invertibility() {
        assert!(
            !matrix_4x4::from_2di([
                [1, 2, 3, 4],
                [2, 4, 6, 8],
                [1, 0, 1, 0],
                [0, 1, 0, 1]
            ]).is_invertible(),
        );
        assert!(
            matrix_4x4::from_2di([
                [2, 0, 1, 3],
                [1, 2, 0, 4],
                [0, 5, 1, 2],
                [3, 1, 0, 1]
            ]).is_invertible()
        );
        assert!(
            !matrix_4x4::from_2di([
                [1, 0, 0, 0],
                [0, 2, 0, 0],
                [0, 0, 3, 0],
                [0, 0, 0, 0]
            ]).is_invertible()
        )
    }

    #[rustfmt::skip]
    #[test]
    fn test_inverses() {
        assert_eq!(
            matrix_4x4::from_2di([
                [1, 2, 0, 3],
                [0, 1, 4, 2],
                [5, 0, 1, 1],
                [2, 3, 0, 1]
            ]).inverse(),
            Some(matrix_4x4::from_2df([
                [-1.0 / 21.0, -1.0 / 21.0, 4.0 / 21.0, 1.0 / 21.0],
                [-16.0 / 147.0, 5.0 / 147.0, -20.0 / 147.0, 58.0 / 147.0],
                [-9.0 / 49.0, 12.0 / 49.0, 1.0 / 49.0, 2.0 / 49.0],
                [62.0 / 147.0, -1.0 / 147.0, 4.0 / 147.0, -41.0 / 147.0]
            ]))
        );
        assert_eq!(
            matrix_4x4::from_2di([
                [3, 0, 2, 1],
                [1, 2, 0, 1],
                [0, 1, 1, 0],
                [2, 3, 0, 1]
            ]).inverse(),
            Some(matrix_4x4::from_2df([
                [0.16667, -0.83333, -0.33333, 0.66667],
                [-0.16667, -0.16667, 0.33333, 0.33333],
                [0.16667, 0.16667, 0.66667, -0.33333],
                [0.16667, 2.16667, -0.33333, -1.33333]
            ]))
        )
    }
}
