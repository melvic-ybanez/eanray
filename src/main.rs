use crate::dsl::ast::Scene;
use config::{Config, File};
use std::io;
use std::io::Read;

mod core;
mod dsl;
mod settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Config::builder()
        .add_source(File::with_name("config"))
        .build()?
        .try_deserialize::<settings::Config>()?;

    let settings: &'static settings::Config = Box::leak(Box::new(settings));

    let mut raw_scene = String::new();
    io::stdin().read_to_string(&mut raw_scene)?;
    let scene: Scene = serde_json::from_str(&raw_scene)?;
    match scene.build(settings) {
        Ok((camera, world)) => camera.render(world, settings)?,
        Err(message) => println!("{message}"),
    }

    Ok(())
}
