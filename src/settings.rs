use crate::core::math::Real;
use serde::Deserialize;

pub type Vec3D = [Real; 3];
pub type Point = Vec3D;
pub type Color = Vec3D;

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
    diagnostics: DiagnosticsConfig,
}

impl AppConfig {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn scene(&self) -> &SceneConfig {
        &self.scene
    }

    pub fn diagnostics(&self) -> &DiagnosticsConfig {
        &self.diagnostics
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
    samples_per_pixel: u32,
    antialiasing: bool,
    max_depth: u32,
    field_of_view: Real,
    look_from: Point,
    look_at: Point,
    vup: Vec3D,
    defocus_angle: Real,
    focus_distance: Real,
    background: Color,
    tile_width: u32,
    tile_height: u32,
}

impl CameraDefaults {
    pub fn samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }

    pub fn antialiasing(&self) -> bool {
        self.antialiasing
    }

    pub fn max_depth(&self) -> u32 {
        self.max_depth
    }

    pub fn field_of_view(&self) -> Real {
        self.field_of_view
    }

    pub fn look_from(&self) -> Point {
        self.look_from
    }

    pub fn look_at(&self) -> Point {
        self.look_at
    }

    pub fn vup(&self) -> Vec3D {
        self.vup
    }

    pub fn defocus_angle(&self) -> Real {
        self.defocus_angle
    }

    pub fn focus_distance(&self) -> Real {
        self.focus_distance
    }

    pub fn background(&self) -> Color {
        self.background
    }

    pub fn tile_width(&self) -> u32 {
        self.tile_width
    }

    pub fn tile_height(&self) -> u32 {
        self.tile_height
    }
}

#[derive(Deserialize, Clone)]
pub struct DiagnosticsConfig {
    enable_metrics: bool,
    enable_stats: bool,
}

impl DiagnosticsConfig {
    pub fn enable_metrics(&self) -> bool {
        self.enable_metrics
    }

    pub fn enable_stats(&self) -> bool {
        self.enable_stats
    }
}
