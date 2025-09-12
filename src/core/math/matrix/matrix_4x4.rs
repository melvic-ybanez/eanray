use crate::core::math;
use crate::core::math::{Matrix, Real};

#[rustfmt::skip]
pub(crate) fn identity() -> Matrix {
    from_arrayi([
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1,
    ])
}

#[rustfmt::skip]
pub(crate) fn translation(x: Real, y: Real, z: Real) -> Matrix {
    from_arrayf([
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0
    ])
}

#[rustfmt::skip]
pub(crate) fn scaling(x: Real, y: Real, z: Real) -> Matrix {
    from_arrayf([
        x, 0.0, 0.0, 0.0,
        0.0, y, 0.0, 0.0,
        0.0, 0.0, z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

/// Rotation around x is defined by the following formulas:
///     1. `y = cos(theta) * y - sin(theta) * z`
///     2. `z = sin(theta) * y + cos(theta) * z`
#[rustfmt::skip]
pub(crate) fn rotation_x(theta: Real) -> Matrix {
    rotation(theta, |theta_cos, theta_sin| [
        1.0, 0.0, 0.0, 0.0,
        0.0, theta_cos, -theta_sin, 0.0,
        0.0, theta_sin, theta_cos, 0.0,
        0.0, 0.0, 0.0, 1.0
    ])
}

/// Rotation around y is defined by the following formulas:
///     1. `x = cos(theta) * x + sin(theta) * z`
///     2. `z = -sin(theta) * x + cos(theta) * z`
#[rustfmt::skip]
pub(crate) fn rotation_y(theta: Real) -> Matrix {
    rotation(theta, |theta_cos, theta_sin| [
        theta_cos, 0.0, theta_sin, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -theta_sin, 0.0, theta_cos, 0.0,
        0.0, 0.0, 0.0, 1.0
    ])
}

/// Rotation around z is defined by the following formulas:
///     1. `x = cos(theta) * x - sin(theta) * y`
///     2. `y = sin(theta) * x + cos(theta) * y`
#[rustfmt::skip]
pub(crate) fn rotation_z(theta: Real) -> Matrix {
    rotation(theta, |theta_cos, theta_sin| [
        theta_cos, -theta_sin, 0.0, 0.0,
        theta_sin, theta_cos, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

fn rotation<F>(theta: Real, f: F) -> Matrix
where
    F: FnOnce(Real, Real) -> [Real; 16],
{
    let theta = math::degrees_to_radians(theta);
    let theta_cos = theta.cos();
    let theta_sin = theta.sin();

    from_arrayf(f(theta_cos, theta_sin))
}

#[rustfmt::skip]
pub(crate) fn shearing(xy: Real, xz: Real, yx: Real, yz: Real, zx: Real, zy: Real) -> Matrix {
    from_arrayf([
        1.0, xy, xz, 0.0,
        yz, 1.0, yz, 0.0,
        zx, zy, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ])
}

pub(crate) fn fill(value: Real) -> Matrix {
    Matrix::fill(4, value)
}

pub(crate) fn fill_default() -> Matrix {
    Matrix::fill_default(4)
}

pub(crate) fn from_arrayf(elems: [Real; 16]) -> Matrix {
    Matrix::from_vec(elems.to_vec())
}

pub(crate) fn from_arrayi(elems: [i32; 16]) -> Matrix {
    from_arrayf(elems.map(|e| e as Real))
}

pub(crate) fn from_2df(elems: [[Real; 4]; 4]) -> Matrix {
    from_arrayf(elems.concat().try_into().unwrap())
}

pub(crate) fn from_2di(elems: [[i32; 4]; 4]) -> Matrix {
    from_2df(elems.map(|row| row.map(|f| f as Real)))
}
