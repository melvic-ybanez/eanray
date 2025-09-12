use crate::core::math::Matrix;
use crate::core::math::matrix::matrix_4x4;

#[rustfmt::skip]
#[test]
fn test_multiplications() {
    let a = matrix_4x4::from_2di([
        [1, 2, 3, 4],
        [0, 1, 0, 1],
        [2, 0, 1, 0],
        [1, 1, 1, 1]
    ]);
    let b = matrix_4x4::from_2di([
        [1, 0, 2, 1],
        [0, 1, 0, 2],
        [3, 0, 1, 0],
        [2, 1, 0, 1]
    ]);
    assert_eq!(a * b, matrix_4x4::from_2di([
        [18, 6, 5, 9],
        [2, 2, 0, 3],
        [5, 0, 5, 2],
        [6, 2, 3, 4]
    ]));

    let a = matrix_4x4::from_2di([
        [2, 1, 0, 3],
        [1, 0, 1, 2],
        [0, 2, 1, 1],
        [3, 1, 0, 0]
    ]);
    let b = matrix_4x4::from_2di([
        [1, 2, 0, 1],
        [0, 1, 1, 0],
        [2, 0, 1, 2],
        [1, 1, 0, 1]
    ]);
    assert_eq!(a * b, matrix_4x4::from_2di([
        [5, 8, 1, 5],
        [5, 4, 1, 5],
        [3, 3, 3, 3],
        [3, 7, 1, 3]
    ]))
}

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
    let mut matrix = matrix_4x4::from_2di([
        [1, 2, 3, 4],
        [2, 0, 1, 5],
        [3, 1, 2, 6],
        [4, 2, 1, 0]
    ]);
    assert_eq!(matrix.determinant(), -3.0);

    let mut matrix = matrix_4x4::from_2di([
        [2, 1, 0, 3],
        [4, 5, 6, 1],
        [7, 0, 8, 2],
        [1, 2, 3, 4]
    ]);
    assert_eq!(matrix.determinant(), 450.0);

    let mut matrix = matrix_4x4::from_2di([
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

#[test]
fn test_transposition() {
    assert_eq!(matrix_4x4::identity().transpose(), matrix_4x4::identity());

    assert_eq!(
        matrix_4x4::from_2di([[1, 2, 3, 4], [0, 1, 0, 1], [5, 6, 7, 8], [9, 0, 1, 2]])
            .transpose(),
        matrix_4x4::from_2di([[1, 0, 5, 9], [2, 1, 6, 0], [3, 0, 7, 1], [4, 1, 8, 2]])
    );

    assert_eq!(
        matrix_4x4::from_2di([[2, 3, 1, 0], [4, 5, 6, 1], [7, 0, 8, 2], [1, 2, 3, 4]])
            .transpose(),
        matrix_4x4::from_2di([[2, 4, 7, 1], [3, 5, 0, 2], [1, 6, 8, 3], [0, 1, 2, 4]])
    )
}