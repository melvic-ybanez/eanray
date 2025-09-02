use crate::common::macros::impl_index;
use crate::core::math::tuple::{Elems, Tuple4};
use crate::core::math::Real;
use std::default::Default;
use std::ops::{Index, IndexMut, Mul};

type Table4x4 = [Real; 16];
type Table3x3 = [Real; 9];
type Table2x2 = [Real; 4];

/// Represents a 4x4 matrix, as the name suggests.
/// Note that there is no need to generalize the matrix to have arbitrary size because
/// most of the operations we care about here only involve 4x4 matrices.
#[derive(Clone, Debug)]
pub(crate) struct Matrix4x4 {
    elems: Table4x4,
}

impl Matrix4x4 {
    //noinspection RsConstNaming
    #[rustfmt::skip]
    pub(crate) const IDENTITY_4x4: Matrix4x4 = Self {
        elems: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ],
    };

    pub(crate) fn from_array(elems: Table4x4) -> Self {
        Self { elems }
    }

    pub(crate) fn from_2df(elems: [[Real; 4]; 4]) -> Self {
        Self::from_array(elems.concat().try_into().unwrap())
    }

    pub(crate) fn from_2di(elems: [[u32; 4]; 4]) -> Self {
        Self::from_2df(elems.map(|row| row.map(|f| f as Real)))
    }

    pub(crate) fn transpose(&self) -> Self {
        let mut transposition: Matrix4x4 = self.clone();
        matrix_loop(|row, col| {
            transposition[(row, col)] = self[(col, row)];
        });
        transposition
    }

    /// Yields a new smaller matrix that does not contain the
    /// row `row` and column `col`.
    fn submatrix(&self, row: usize, col: usize) -> Table3x3 {
        let mut table = Table3x3::default();
        let mut i = 0;
        matrix_loop(|r, c| {
            if r != row && c != col {
                table[i] = self[(r, c)];
                i += 1;
            }
        });
        table
    }

    fn determinant(&self) -> Real {
        let mut det = 0.0;
        for col in 0..4 {
            det += self[(0, col)] * self.cofactor(0, col);
        }
        det
    }

    fn cofactor(&self, row: usize, col: usize) -> Real {
        cofactor_from_minor(self.minor(row, col), row, col)
    }

    /// The minor at (`row`, `col`) of a 4x4 matrix, defined as
    /// the determinant of the submatrix at (`row`, `col`).
    fn minor(&self, row: usize, col: usize) -> Real {
        matrix_3x3::determinant(self.submatrix(row, col))
    }

    fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    fn inverse(&self) -> Option<Matrix4x4> {
        if self.is_invertible() {
            let mut matrix = Matrix4x4::default();
            let determinant = self.determinant();

            matrix_loop(|row, col| {
                let c = self.cofactor(row, col);

                // we are reversing `row` and `col` to transpose the matrix
                matrix[(col, row)] = c / determinant;
            });

            Some(matrix)
        } else {
            None
        }
    }
}
mod matrix_3x3 {
    use super::*;

    pub(super) fn determinant(matrix: Table3x3) -> Real {
        let mut det = 0.0;
        for col in 0..3 {
            det += matrix[fold_index((0, col), 3)] * cofactor(matrix, 0, col);
        }
        det
    }

    fn submatrix(matrix_3x3: Table3x3, row: usize, col: usize) -> Table2x2 {
        let mut table = Table2x2::default();
        let mut i = 0;

        for index in 0..9 {
            let (r, c) = unfold_index(index, 3);
            if r != row && c != col {
                table[i] = matrix_3x3[index];
                i += 1;
            }
        }

        table
    }

    fn minor(matrix_3x3: Table3x3, row: usize, col: usize) -> Real {
        determinant_2x2(submatrix(matrix_3x3, row, col))
    }

    fn cofactor(matrix_3x3: Table3x3, row: usize, col: usize) -> Real {
        let minor = minor(matrix_3x3, row, col);
        cofactor_from_minor(minor, row, col)
    }
}

fn determinant_2x2(matrix_2x2: Table2x2) -> Real {
    let a = matrix_2x2[fold_index((0, 0), 2)];
    let b = matrix_2x2[fold_index((0, 1), 2)];
    let c = matrix_2x2[fold_index((1, 0), 2)];
    let d = matrix_2x2[fold_index((1, 1), 2)];

    a * d - b * c
}

fn cofactor_from_minor(minor: Real, row: usize, col: usize) -> Real {
    if (row + col) % 2 == 0 { minor } else { -minor }
}

fn fold_index(index: (usize, usize), size: usize) -> usize {
    let (row, col) = index;
    row * size + col
}

fn unfold_index(index: usize, size: usize) -> (usize, usize) {
    (index / size, index % size)
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self {
            elems: Table4x4::default(),
        }
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..16 {
            if (self[i] - other[i]).abs() >= 1e-5 {
                return false;
            }
        }
        true
    }
}

impl_index!(usize, Matrix4x4, Real, elems);

impl Index<(usize, usize)> for Matrix4x4 {
    type Output = Real;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self[fold_index(index, 4)]
    }
}

impl IndexMut<(usize, usize)> for Matrix4x4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self[fold_index(index, 4)]
    }
}

impl Mul<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: &Matrix4x4) -> Self::Output {
        let mut result: Matrix4x4 = Matrix4x4::default();
        matrix_loop(|row, col| {
            // this is effectively a series of dot products
            for i in 0..4 {
                result[(row, col)] += self[(row, i)] * other[(i, col)];
            }
        });
        result
    }
}

impl Mul<&Tuple4> for &Matrix4x4 {
    type Output = Tuple4;

    fn mul(self, tuple: &Tuple4) -> Self::Output {
        let mut result = Elems::default();
        for row in 0..4 {
            // like dot product, but for a single column
            for i in 0..4 {
                result[row] += self[(row, i)] * tuple[i];
            }
        }
        Tuple4::from_elems(result)
    }
}

fn matrix_loop<F>(mut f: F)
where
    F: FnMut(usize, usize) -> (),
{
    for row in 0..4 {
        for col in 0..4 {
            f(row, col);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn test_submatrices() {
        let matrix = Matrix4x4::from_2di([
            [1, 2, 3, 4],
            [0, 5, 6, 7],
            [8, 9, 1, 2],
            [3, 4, 5, 6]
        ]);

        let submatrix = [
            [1, 2, 4],
            [8, 9, 2],
            [3, 4, 6]
        ];

        assert_eq!(matrix.submatrix(1, 2), to_table_3x3_f(submatrix));

        let matrix = Matrix4x4::from_2di([
            [4, 1, 0, 2],
            [7, 5, 9, 3],
            [6, 8, 2, 1],
            [5, 0, 3, 4]
        ]);

        let submatrix = [
            [5, 9, 3],
            [8, 2, 1],
            [0, 3, 4]
        ];

        assert_eq!(matrix.submatrix(0, 0), to_table_3x3_f(submatrix));
    }

    #[rustfmt::skip]
    #[test]
    fn test_determinants() {
        let matrix = Matrix4x4::from_2di([
            [1, 2, 3, 4],
            [2, 0, 1, 5],
            [3, 1, 2, 6],
            [4, 2, 1, 0]
        ]);
        assert_eq!(matrix.determinant(), -3.0);

        let matrix = Matrix4x4::from_2di([
            [2, 1, 0, 3],
            [4, 5, 6, 1],
            [7, 0, 8, 2],
            [1, 2, 3, 4]
        ]);
        assert_eq!(matrix.determinant(), 450.0);

        let matrix = Matrix4x4::from_2di([
            [0, 2, 1, 3],
            [4, 1, 5, 2],
            [7, 0, 6, 1],
            [3, 2, 4, 0]
        ]);
        assert_eq!(matrix.determinant(), -51.0);
    }

    #[rustfmt::skip]
    #[test]
    fn test_minors_and_cofactors() {
        let matrix = Matrix4x4::from_2di([
            [1, 2, 3, 4],
            [0, 5, 6, 7],
            [8, 9, 1, 2],
            [3, 4, 5, 6]
        ]);
        assert_eq!(matrix.minor(0, 0), -9.0);
        assert_eq!(matrix.cofactor(0, 0), -9.0);

        let matrix = Matrix4x4::from_2di([
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
            !Matrix4x4::from_2di([
                [1, 2, 3, 4],
                [2, 4, 6, 8],
                [1, 0, 1, 0],
                [0, 1, 0, 1]
            ]).is_invertible(),
        );
        assert!(
            Matrix4x4::from_2di([
                [2, 0, 1, 3],
                [1, 2, 0, 4],
                [0, 5, 1, 2],
                [3, 1, 0, 1]
            ]).is_invertible()
        );
        assert!(
            !Matrix4x4::from_2di([
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
            Matrix4x4::from_2di([
                [1, 2, 0, 3],
                [0, 1, 4, 2],
                [5, 0, 1, 1],
                [2, 3, 0, 1]
            ]).inverse(),
            Some(Matrix4x4::from_2df([
                [-1.0 / 21.0, -1.0 / 21.0, 4.0 / 21.0, 1.0 / 21.0],
                [-16.0 / 147.0, 5.0 / 147.0, -20.0 / 147.0, 58.0 / 147.0],
                [-9.0 / 49.0, 12.0 / 49.0, 1.0 / 49.0, 2.0 / 49.0],
                [62.0 / 147.0, -1.0 / 147.0, 4.0 / 147.0, -41.0 / 147.0]
            ]))
        );
        assert_eq!(
            Matrix4x4::from_2di([
                [3, 0, 2, 1],
                [1, 2, 0, 1],
                [0, 1, 1, 0],
                [2, 3, 0, 1]
            ]).inverse(),
            Some(Matrix4x4::from_2df([
                [0.16667, -0.83333, -0.33333, 0.66667],
                [-0.16667, -0.16667, 0.33333, 0.33333],
                [0.16667, 0.16667, 0.66667, -0.33333],
                [0.16667, 2.16667, -0.33333, -1.33333]
            ]))
        )
    }

    fn to_table_3x3_f(table: [[u32; 3]; 3]) -> Table3x3 {
        table
            .map(|row| row.map(|f| f as Real))
            .concat()
            .try_into()
            .unwrap()
    }
}
