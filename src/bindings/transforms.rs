use crate::bindings::lua;
use crate::bindings::lua::from_user_data;
use crate::core::math::{Real, Vec3D};
use crate::core::transforms::{Rotate, RotateKind, Translate};
use crate::core::Hittable;
use mlua::{AnyUserData, Lua, LuaSerdeExt, Table, Value};
use std::sync::Arc;

pub(crate) fn new_table(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("Translate", new_translate_table(lua)?)?;
    table.set("RotateX", new_rotate_table(lua, RotateKind::X)?)?;
    table.set("RotateY", new_rotate_table(lua, RotateKind::Y)?)?;
    table.set("RotateZ", new_rotate_table(lua, RotateKind::Z)?)?;
    Ok(table)
}

fn new_translate_table(lua: &Lua) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(|lua, (_, object, offset): (Table, Value, AnyUserData)| {
            let object: Hittable = lua.from_value(object)?;
            let offset = from_user_data!(offset, Vec3D);
            let translate = Hittable::Translate(Translate::new(Arc::new(object), offset));
            Ok(lua.to_value(&translate))
        }),
    )
}

fn new_rotate_table(lua: &Lua, kind: RotateKind) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(move |lua, (_, object, angle): (Table, Value, Real)| {
            let object: Hittable = lua.from_value(object)?;
            let rotate_y = Hittable::Rotate(Rotate::new(Arc::new(object), angle, kind.clone()));
            Ok(lua.to_value(&rotate_y))
        }),
    )
}
