use mlua::{Lua, LuaSerdeExt, Table, UserData};
use crate::bindings::lua;
use crate::core::math;
use crate::core::math::{Real, VecLike};
use crate::core::math::vector::{PointKind, VecKind};

pub(crate) fn new_vec_like_table<K: 'static>(lua: &Lua) -> mlua::Result<Table>
where
    VecLike<K>: UserData,
{
    let table = lua::new_table(
        lua,
        lua.create_function(|lua, (_, x, y, z): (Table, Real, Real, Real)| {
            let vec_like: VecLike<K> = VecLike::<K>::new(x, y, z);
            Ok(lua.create_ser_userdata(vec_like))
        }),
    )?;

    table.set("ZERO", lua.to_value(&VecLike::<K>::zero())?)?;
    table.set(
        "random",
        lua.create_function(|lua, ()| {
            let vec_like: VecLike<K> = VecLike::<K>::random();
            Ok(lua.create_ser_userdata(vec_like))
        })?,
    )?;
    table.set(
        "random_range",
        lua.create_function(|lua, (min, max): (Real, Real)| {
            let vec_like: VecLike<K> = VecLike::<K>::random_range(min, max);
            Ok(lua.create_ser_userdata(vec_like))
        })?,
    )?;

    Ok(table)
}

pub(crate) fn new_table(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;

    table.set("Vec", new_vec_like_table::<VecKind>(lua)?)?;

    table.set("Point", new_vec_like_table::<PointKind>(lua)?)?;
    table.set(
        "random",
        lua.create_function(|_, ()| Ok(math::random_real()))?,
    )?;
    table.set(
        "random_range",
        lua.create_function(|_, (min, max)| Ok(math::random_range(min, max)))?,
    )?;
    Ok(table)
}