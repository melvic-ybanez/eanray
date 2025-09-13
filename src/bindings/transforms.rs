use crate::bindings::lua;
use crate::bindings::macros::from_user_data;
use crate::core::math::matrix::matrix_4x4;
use crate::core::math::{Matrix, Real};
use mlua::{AnyUserData, Lua, ObjectLike, Table, UserData, UserDataMethods};

pub(crate) fn new_table(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("Translate", new_translate_table(lua)?)?;
    table.set("RotateX", new_rotate_table(lua, matrix_4x4::rotation_x)?)?;
    table.set("RotateY", new_rotate_table(lua, matrix_4x4::rotation_y)?)?;
    table.set("RotateZ", new_rotate_table(lua, matrix_4x4::rotation_z)?)?;
    table.set("Scale", new_scale_table(lua)?)?;
    Ok(table)
}

fn new_translate_table(lua: &Lua) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(|_, (_, x, y, z): (Table, Real, Real, Real)| {
            Ok(matrix_4x4::translation(x, y, z))
        }),
    )
}

fn new_rotate_table(lua: &Lua, f: fn(Real) -> Matrix) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(move |_, (_, angle): (Table, Real)| Ok(f(angle))),
    )
}

fn new_scale_table(lua: &Lua) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(|_, (_, x, y, z): (Table, Real, Real, Real)| {
            Ok(matrix_4x4::scaling(x, y, z))
        }),
    )
}

impl UserData for Matrix {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("and_then", |_, this, transform: AnyUserData| {
            let next_transform = from_user_data!(transform, Matrix);
            let product = &next_transform * this;
            Ok(product)
        });
    }
}
