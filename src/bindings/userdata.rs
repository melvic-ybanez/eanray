use crate::bindings::macros::from_user_data;
use crate::core::camera::Background;
use crate::core::math::vector::CanAdd;
use crate::core::math::{Point, Real, Vec3D, VecLike};
use crate::core::{Color, Material};
use mlua::{
    AnyUserData, MetaMethod, UserData, UserDataFields, UserDataMethods, Value,
};
use crate::core::textures::Texture;

impl UserData for Vec3D {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        add_common_vec_like_fields(fields);
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_addable_vec_methods(methods);
        methods.add_method("length", |_, this, ()| Ok(this.length()))
    }
}

impl UserData for Color {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        add_common_vec_like_fields(fields);
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_addable_vec_methods(methods);
    }
}

impl UserData for Point {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        add_common_vec_like_fields(fields);
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Sub, |lua, this, other: AnyUserData| {
            let point: Point = from_user_data!(other, Point);
            Ok(this - point)
        });
        methods.add_meta_method(MetaMethod::Add, |lua, this, other: AnyUserData| {
            let other_vec3: Vec3D = from_user_data!(other, Vec3D);
            Ok(this + other_vec3)
        });
    }
}

fn add_addable_vec_methods<K: 'static + CanAdd + Clone, M: UserDataMethods<VecLike<K>>>(
    methods: &mut M,
) where
    VecLike<K>: UserData,
{
    methods.add_meta_method(MetaMethod::Mul, |_, this, rhs| match rhs {
        Value::Integer(scalar) => Ok(this * scalar as Real),
        Value::Number(scalar) => Ok(this * scalar),
        Value::UserData(userdata) => {
            let other_vec_like: VecLike<K> = userdata.borrow::<VecLike<K>>()?.clone();
            Ok(this * other_vec_like)
        }
        _ => Err(mlua::Error::RuntimeError("Invalid RHS".into())),
    });
    methods.add_meta_method(MetaMethod::Add, |lua, this, other: AnyUserData| {
        let other_vec_like: VecLike<K> = from_user_data!(other, VecLike<K>);
        Ok(this + other_vec_like)
    });
}

fn add_common_vec_like_fields<K: 'static + Clone, F: UserDataFields<VecLike<K>>>(fields: &mut F)
where
    VecLike<K>: UserData,
{
    fields.add_field_method_get("x", |_, this| Ok(this.x()));
    fields.add_field_method_get("y", |_, this| Ok(this.y()));
    fields.add_field_method_get("z", |_, this| Ok(this.z()));
}

impl UserData for Background {}

impl UserData for Texture {}

impl UserData for Material {}
