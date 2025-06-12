use crate::interface::lua::SceneSchema;
use config::{Config, File};
use mlua::{Lua, LuaSerdeExt};
use serde::Serialize;
use std::{fs, io};
use std::io::Read;

mod core;
pub mod interface;
mod settings;

fn main() -> mlua::Result<()> {
    let mut scene_script = String::new();
    io::stdin().read_to_string(&mut scene_script)?;

    let lua = Lua::new();
    interface::lua::set_engine(&lua)?;
    
    let helpers = fs::read_to_string("scripts/helpers.lua")?;
    lua.load(&helpers).exec()?;

    let scene_table = lua.load(scene_script).eval()?;
    let scene: SceneSchema = lua.from_value(scene_table)?;

    let settings = Config::builder()
        .add_source(File::with_name("config"))
        .build()
        .map_err(mlua::Error::external)?
        .try_deserialize::<settings::Config>()
        .map_err(mlua::Error::external)?;

    let settings: &'static settings::Config = Box::leak(Box::new(settings));

    scene.render(settings).map_err(mlua::Error::external)
}
