use crate::bindings::lua;
use crate::bindings::macros::from_user_data;
use crate::core::materials::{refractive_index, Dielectric, DiffuseLight, Lambertian, Metal};
use crate::core::math::Real;
use crate::core::textures::Texture;
use crate::core::{Color, Material};
use mlua::{AnyUserData, Lua, Table};

pub(crate) fn new_table(lua: &Lua) -> mlua::Result<Table> {
    let materials = lua.create_table()?;
    materials.set("Lambertian", new_lambertian_table(lua)?)?;
    materials.set("Metal", new_metal_table(lua)?)?;
    materials.set("Dielectric", new_dielectric_table(lua)?)?;
    materials.set("DiffuseLight", new_diffuse_light_table(lua)?)?;
    Ok(materials)
}

fn new_lambertian_table(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set(
        "from_texture",
        lua.create_function(|lua, (_, texture): (Table, AnyUserData)| {
            let texture: Texture = from_user_data!(texture, Texture);
            Ok(Material::Lambertian(Lambertian::from_texture(texture)))
        })?,
    )?;

    table.set(
        "from_albedo",
        lua.create_function(|lua, (_, albedo): (Table, AnyUserData)| {
            let albedo: Color = from_user_data!(albedo, Color);
            Ok(Material::Lambertian(Lambertian::from_albedo(albedo)))
        })?,
    )?;

    Ok(table)
}

fn new_metal_table(lua: &Lua) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(|_, (_, albedo, fuzz): (Table, AnyUserData, Real)| {
            let albedo: Color = from_user_data!(albedo, Color);
            Ok(Material::Metal(Metal::new(albedo, fuzz)))
        }),
    )
}

fn new_dielectric_table(lua: &Lua) -> mlua::Result<Table> {
    let table = lua::new_table(
        lua,
        lua.create_function(|_, (_, refraction_index): (Table, Real)| {
            Ok(Material::Dielectric(Dielectric::new(refraction_index)))
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
        lua.create_function(|_, (_, emission_color): (Table, AnyUserData)| {
            let emission_color: Color = from_user_data!(emission_color, Color);
            Ok(Material::DiffuseLight(DiffuseLight::from_emission(
                emission_color,
            )))
        })?,
    )?;
    table.set(
        "from_texture",
        lua.create_function(|_, (_, texture): (Table, AnyUserData)| {
            let texture: Texture = from_user_data!(texture, Texture);
            Ok(Material::DiffuseLight(DiffuseLight::from_texture(texture)))
        })?,
    )?;
    table.set(
        "from_texture_intensified",
        lua.create_function(
            |_, (_, texture, intensity): (Table, AnyUserData, AnyUserData)| {
                let texture: Texture = from_user_data!(texture, Texture);
                let intensity = from_user_data!(intensity, Color);
                let diffuse_light = Material::DiffuseLight(DiffuseLight::from_texture_intensified(
                    texture, intensity,
                ));
                Ok(diffuse_light)
            },
        )?,
    )?;
    table.set(
        "from_emission_intensified",
        lua.create_function(
            |_, (_, emission_color, intensity): (Table, AnyUserData, AnyUserData)| {
                let emission_color: Color = from_user_data!(emission_color, Color);
                let intensity = from_user_data!(intensity, Color);
                let diffuse_light = Material::DiffuseLight(
                    DiffuseLight::from_emission_intensified(emission_color, intensity),
                );
                Ok(diffuse_light)
            },
        )?,
    )?;

    Ok(table)
}
