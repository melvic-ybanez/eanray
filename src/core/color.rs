use std::fmt;
use std::fmt::Formatter;
use crate::core::math::Real;
use crate::core::math::interval::Interval;
use crate::core::math::vector::{CanAdd, Vec3D, VecLike};
use std::fs::File;
use std::io::{self, Write};

pub struct ColorKind;

pub type Color = VecLike<ColorKind>;

impl Color {
    pub fn red_component(&self) -> Real {
        self.x
    }

    pub fn green_component(&self) -> Real {
        self.y
    }

    pub fn blue_component(&self) -> Real {
        self.z
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }
    
    pub fn to_bytes_string(&self) -> String {
        let intensity = Interval::new(0.0, 0.999);
        let r_byte = (intensity.clamp(self.red_component()) * 256.0) as u16;
        let g_byte = (intensity.clamp(self.green_component()) * 256.0) as u16;
        let b_byte = (intensity.clamp(self.blue_component()) * 256.0) as u16;

        format!("{} {} {}", r_byte, g_byte, b_byte)
    }
}

impl CanAdd for ColorKind {}

impl From<Vec3D> for Color {
    fn from(v: Vec3D) -> Self {
        Color::new(v.x, v.y, v.z)
    }
}