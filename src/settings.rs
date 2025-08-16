use crate::core::math::Real;
use serde::Deserialize;

pub(crate) type Vec3D = [Real; 3];
pub(crate) type Point = Vec3D;
pub(crate) type Color = Vec3D;

#[derive(Deserialize, Clone)]
pub(crate) struct Config {
    app: AppConfig,
}

impl Config {
    pub(crate) fn app(&self) -> &AppConfig {
        &self.app
    }
}

#[derive(Deserialize, Clone)]
pub(crate) struct AppConfig {
    name: String,
    scene: SceneConfig,
    diagnostics: DiagnosticsConfig,
}

impl AppConfig {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn scene(&self) -> &SceneConfig {
        &self.scene
    }

    pub(crate) fn diagnostics(&self) -> &DiagnosticsConfig {
        &self.diagnostics
    }
}

#[derive(Deserialize, Clone)]
pub(crate) struct SceneConfig {
    output_file: String,
    camera: CameraConfig,
}

impl SceneConfig {
    pub(crate) fn camera(&self) -> &CameraConfig {
        &self.camera
    }

    pub(crate) fn output_file(&self) -> &str {
        &self.output_file
    }
}

#[derive(Deserialize, Clone)]
pub(crate) struct CameraConfig {
    defaults: CameraDefaults,
}

impl CameraConfig {
    pub(crate) fn defaults(&self) -> &CameraDefaults {
        &self.defaults
    }
}

#[derive(Deserialize, Clone)]
pub(crate) struct CameraDefaults {
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
    pub(crate) fn samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }

    pub(crate) fn antialiasing(&self) -> bool {
        self.antialiasing
    }

    pub(crate) fn max_depth(&self) -> u32 {
        self.max_depth
    }

    pub(crate) fn field_of_view(&self) -> Real {
        self.field_of_view
    }

    pub(crate) fn look_from(&self) -> Point {
        self.look_from
    }

    pub(crate) fn look_at(&self) -> Point {
        self.look_at
    }

    pub(crate) fn vup(&self) -> Vec3D {
        self.vup
    }

    pub(crate) fn defocus_angle(&self) -> Real {
        self.defocus_angle
    }

    pub(crate) fn focus_distance(&self) -> Real {
        self.focus_distance
    }

    pub(crate) fn background(&self) -> Color {
        self.background
    }

    pub(crate) fn tile_width(&self) -> u32 {
        self.tile_width
    }

    pub(crate) fn tile_height(&self) -> u32 {
        self.tile_height
    }
}

#[derive(Deserialize, Clone)]
pub(crate) struct DiagnosticsConfig {
    enable_metrics: bool,
    enable_stats: bool,
}

impl DiagnosticsConfig {
    pub(crate) fn enable_metrics(&self) -> bool {
        self.enable_metrics
    }

    pub(crate) fn enable_stats(&self) -> bool {
        self.enable_stats
    }
}
