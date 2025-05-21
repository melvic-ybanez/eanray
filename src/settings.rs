use crate::core::math::Real;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    app: AppConfig,
}

impl Config {
    pub fn app(&self) -> &AppConfig {
        &self.app
    }
}

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    name: String,
    scene: SceneConfig,
}

impl AppConfig {
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn scene(&self) -> &SceneConfig {
        &self.scene
    }
}

#[derive(Deserialize, Clone)]
pub struct SceneConfig {
    output_file: String,
    camera: CameraConfig,
}

impl SceneConfig {
    pub fn camera(&self) -> &CameraConfig {
        &self.camera
    }
    
    pub fn output_file(&self) -> &str {
        &self.output_file
    }
}

#[derive(Deserialize, Clone)]
pub struct CameraConfig {
    defaults: CameraDefaults,
}

impl CameraConfig {
    pub fn defaults(&self) -> &CameraDefaults {
        &self.defaults
    }
}

#[derive(Deserialize, Clone)]
pub struct CameraDefaults {
    center: [Real; 3],
    focal_length: Real,
    samples_per_pixel: u32,
    antialiasing: bool,
    max_depth: u32,
}

impl CameraDefaults {
    pub fn center(&self) -> [Real; 3] {
        self.center
    }
    
    pub fn focal_length(&self) -> Real {
        self.focal_length
    }
    
    pub fn samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }
    
    pub fn antialiasing(&self) -> bool {
        self.antialiasing
    }
    
    pub fn max_depth(&self) -> u32 {
        self.max_depth
    }
}
