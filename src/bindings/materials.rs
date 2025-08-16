use crate::bindings::lua;
use crate::bindings::lua::from_user_data;
use crate::core::materials::{Dielectric, DiffuseLight, Lambertian, Metal, refractive_index};
use crate::core::math::Real;
use crate::core::textures::Texture;
use crate::core::{Color, Material};
use mlua::{AnyUserData, Lua, LuaSerdeExt, Table, Value};

pub(crate) fn new_table(lua: &Lua) -> mlua::Result<Table> {
    let materials = lua.create_table()?;
    materials.set("Lambertian", new_lambertian_table(lua)?)?;
    materials.set("Metal", new_metal_table(lua)?)?;
    materials.set("Dielectric", new_dielectric_table(lua)?)?;
    materials.set("DiffuseLight", new_diffuse_light_table(lua)?)?;
    Ok(materials)
}

fn new_lambertian_table(lua: &Lua) -> mlua::Result<Table> {
    // TODO: should be `from_texture` instead of `new`
    let table = lua::new_table(
        lua,
        lua.create_function(|lua, (_, texture): (Table, Value)| {
            let texture: Texture = lua.from_value(texture)?;
            let lambertian = Material::Lambertian(Lambertian::new(texture));
            Ok(lua.to_value(&lambertian))
        }),
    )?;

    table.set(
        "from_albedo",
        lua.create_function(|lua, (_, albedo): (Table, AnyUserData)| {
            let albedo: Color = from_user_data!(albedo, Color);
            let lambertian = Material::Lambertian(Lambertian::from_albedo(albedo));
            Ok(lua.to_value(&lambertian))
        })?,
    )?;

    Ok(table)
}

fn new_metal_table(lua: &Lua) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(|lua, (_, albedo, fuzz): (Table, AnyUserData, Real)| {
            let albedo: Color = from_user_data!(albedo, Color);
            let metal = Material::Metal(Metal::new(albedo, fuzz));
            Ok(lua.to_value(&metal))
        }),
    )
}

fn new_dielectric_table(lua: &Lua) -> mlua::Result<Table> {
    let table = lua::new_table(
        lua,
        lua.create_function(|lua, (_, refraction_index): (Table, Real)| {
            let dielectric = Material::Dielectric(Dielectric::new(refraction_index));
            Ok(lua.to_value(&dielectric))
        }),
    )?;

    let refractive_index = lua.create_table()?;
    refractive_index.set("GLASS", refractive_index::GLASS)?;
    refractive_index.set("VACUUM", refractive_index::VACUUM)?;
    refractive_index.set("AIR", refractive_index::AIR)?;
    refractive_index.set("WATER", refractive_index::WATER)?;
    refractive_index.set("DIAMOND", refractive_index::DIAMOND)?;

    table.set("RefractiveIndex", refractive_index)?;

    Ok(table)
}

fn new_diffuse_light_table(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;

    table.set(
        "from_emission",
        lua.create_function(|lua, (_, emission_color): (Table, AnyUserData)| {
            let emission_color: Color = from_user_data!(emission_color, Color);
            let diffuse_light = Material::DiffuseLight(DiffuseLight::from_emission(emission_color));
            Ok(lua.to_value(&diffuse_light))
        })?,
    )?;
    table.set(
        "from_texture",
        lua.create_function(|lua, (_, texture): (Table, Value)| {
            let texture: Texture = lua.from_value(texture)?;
            let diffuse_light = Material::DiffuseLight(DiffuseLight::from_texture(texture));
            Ok(lua.to_value(&diffuse_light))
        })?,
    )?;

    Ok(table)
}
