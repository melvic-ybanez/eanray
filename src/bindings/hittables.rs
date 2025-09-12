use crate::bindings::macros::from_user_data;
use crate::core::math::matrix::matrix_4x4;
use crate::core::math::{Matrix, Real};
use crate::core::transform::Transform;
use crate::core::Hittable;
use mlua::{AnyUserData, Error, UserData, UserDataMethods};
use std::sync::Arc;

impl UserData for Hittable {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        fn build_transform(hittable: &Hittable, transform_matrix: Matrix) -> Result<Hittable, Error> {
            let transform =
                Hittable::Transform(Transform::new(Arc::new(hittable.clone()), transform_matrix));
            Ok(transform)
        }

        methods.add_method("transform", |_, this, transform: AnyUserData| {
            let transform_matrix = from_user_data!(transform, Matrix);
            build_transform(this, transform_matrix)
        });

        methods.add_method("translate", |_, this, (x, y, z): (Real, Real, Real)| {
            let translate = matrix_4x4::translation(x, y, z);
            build_transform(this, translate)
        });

        let mut add_rotate_method = |name: &str, f: fn(Real) -> Matrix| {
            methods.add_method(name, move |_, this, angle: Real| {
                let rotation = f(angle);
                build_transform(this, rotation)
            });
        };

        add_rotate_method("rotate_x", matrix_4x4::rotation_x);
        add_rotate_method("rotate_y", matrix_4x4::rotation_y);
        add_rotate_method("rotate_z", matrix_4x4::rotation_z);
    }
}
