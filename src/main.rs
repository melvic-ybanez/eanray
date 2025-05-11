use std::io;
use std::io::Read;
use crate::dsl::ast::Scene;

mod dsl;
mod core;

fn main() -> io::Result<()> {
    let mut raw_scene = String::new();
    io::stdin().read_to_string(&mut raw_scene)?;
    let scene: Scene = serde_json::from_str(&raw_scene)?;
    let (camera, world) = scene.build();
    camera.render(world)
}
