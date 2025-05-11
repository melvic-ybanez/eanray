use std::io;
use crate::dsl::ast::Scene;

mod dsl;
mod core;

fn main() -> io::Result<()> {
    let raw_scene = r#"{
        "camera": {"aspect_ratio": [16, 9], "image_width": 400},
        "objects": [
            {"sphere": { "center": [0, 0, -1], "radius": 0.5 }},
            {"sphere": { "center": [0, -100.5, -1], "radius": 100.0 }}
        ]
    }"#;
    let scene: Scene = serde_json::from_str(raw_scene)?;
    let (camera, world) = scene.build();
    camera.render(world)
}
