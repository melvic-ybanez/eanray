use crate::bindings::schemas::SceneSchema;
use crate::diagnostics::metrics;
use config::{Config, File};
use mlua::{Lua, LuaSerdeExt};
use std::{env, fs};

pub mod bindings;
mod common;
mod core;
mod diagnostics;
mod settings;

fn main() -> mlua::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        let error_message = "Usage: {} <script_name>";
        eprintln!("{} {}", error_message, args[0]);
        return Err(mlua::Error::external(error_message));
    }

    // TODO: This should be a command line arg
    diagnostics::enable_all(false);

    env_logger::init();
    let lua = Lua::new();

    path_setup(&lua)?;
    engine_setup(&lua)?;

    let script_name = args[1].clone();
    let script_content = fs::read_to_string(script_name)?;

    log::info!("Evaluating Lua script...");
    let scene_table = lua.load(script_content).eval()?;
    let scene: SceneSchema = lua.from_value(scene_table)?;

    log::info!("Script evaluated. Loading configs...");
    let settings = Config::builder()
        .add_source(File::with_name("config"))
        .build()
        .map_err(mlua::Error::external)?
        .try_deserialize::<settings::Config>()
        .map_err(mlua::Error::external)?;

    let settings: &'static settings::Config = Box::leak(Box::new(settings));

    log::info!("Configs loaded. Rendering the scene...");
    let result = scene.render(settings).map_err(mlua::Error::external);

    metrics::report();
    result
}

/// Adds the current directory to the package paths
fn path_setup(lua: &Lua) -> mlua::Result<()> {
    let cwd = env::current_dir()?;
    let scripts_dir = format!("{}/?.lua", cwd.display());
    lua.load(&format!(
        r#"package.path = "{};" .. package.path"#,
        scripts_dir
    ))
    .exec()
}

fn engine_setup(lua: &Lua) -> mlua::Result<()> {
    bindings::lua::set_engine(&lua)?;
    let helpers = fs::read_to_string("scripts/helpers.lua")?;
    lua.load(&helpers).exec()
}
