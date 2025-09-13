use crate::bindings::lua;
use crate::bindings::macros::from_user_data;
use crate::core::math::Real;
use crate::core::textures::{Checker, ImageTexture, NoiseTexture, Texture};
use crate::core::Color;
use mlua::{AnyUserData, Lua, Table};

pub(crate) fn new_table(lua: &Lua) -> mlua::Result<Table> {
    let textures = lua.create_table()?;
    textures.set("Checker", new_checker_table(lua)?)?;
    textures.set("Image", new_image_texture_table(lua)?)?;
    textures.set("Noise", new_noise_texture_table(lua)?)?;
    Ok(textures)
}

fn new_checker_table(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set(
        "from_colors",
        lua.create_function(
            |lua, (_, scale, c1, c2): (Table, Real, AnyUserData, AnyUserData)| {
                let c1 = from_user_data!(c1, Color);
                let c2 = from_user_data!(c2, Color);
                Ok(Texture::Checker(Checker::from_colors(scale, c1, c2)))
            },
        )?,
    )?;

    Ok(table)
}

fn new_image_texture_table(lua: &Lua) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(|lua, (_, filepath): (Table, String)| {
            Ok(Texture::Image(ImageTexture::from_path_unsafe(
                filepath.as_str(),
            )))
        }),
    )
}

fn new_noise_texture_table(lua: &Lua) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(|lua, (_, scale, base_color): (Table, f64, AnyUserData)| {
            let base_color: Color = from_user_data!(base_color, Color);
            Ok(Texture::Noise(NoiseTexture::new(scale, base_color)))
        }),
    )
}
