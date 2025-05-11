use crate::dsl::ast::Scene;
use std::io;
use std::io::Read;

mod core;
mod dsl;

fn main() -> io::Result<()> {
    let mut raw_scene = String::new();
    io::stdin().read_to_string(&mut raw_scene)?;
    let scene: Scene = serde_json::from_str(&raw_scene)?;
    let (camera, world) = scene.build();
    camera.render(world)
}
