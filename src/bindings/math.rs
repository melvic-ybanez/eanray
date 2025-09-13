use crate::bindings::lua;
use crate::core::math;
use crate::core::math::point::PointKind;
use crate::core::math::vector::VecKind;
use crate::core::math::{Real, VecLike};
use mlua::{Lua, Table, UserData};

pub(crate) fn new_vec_like_table<K: 'static>(lua: &Lua) -> mlua::Result<Table>
where
    VecLike<K>: UserData,
{
    let table = lua::new_table(
        lua,
        lua.create_function(|_, (_, x, y, z): (Table, Real, Real, Real)| {
            Ok(VecLike::<K>::new(x, y, z))
        }),
    )?;

    // TODO: This one may not be usable in places where user-data is expected
    table.set("ZERO", VecLike::<K>::zero())?;

    table.set(
        "random",
        lua.create_function(|_, ()| Ok(VecLike::<K>::random()))?,
    )?;
    table.set(
        "random_range",
        lua.create_function(|_, (min, max): (Real, Real)| {
            Ok(VecLike::<K>::random_range(min, max))
        })?,
    )?;
    table.set(
        "from_scalar",
        lua.create_function(|_, (_, scalar): (Table, Real)| Ok(VecLike::from_scalar(scalar)))?,
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
