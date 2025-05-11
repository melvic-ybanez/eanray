use crate::core::math::Real;
use crate::core::math::vector::{CanAdd, Vec3D, VecLike};
use std::fs::File;
use std::io::{self, Write};

pub struct ColorKind;

pub type Color = VecLike<ColorKind>;

impl Color {
    pub fn red_component(&self) -> Real {
        self.x()
    }

    pub fn green_component(&self) -> Real {
        self.y()
    }

    pub fn blue_component(&self) -> Real {
        self.z()
    }

    pub fn write_to_file(&self, mut file: &File) -> io::Result<()> {
        let r_byte = (self.red_component() * 255.999) as u16;
        let g_byte = (self.green_component() * 255.999) as u16;
        let b_byte = (self.blue_component() * 255.999) as u16;

        writeln!(file, "{} {} {}", r_byte, g_byte, b_byte)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }
}

impl CanAdd for ColorKind {}

impl From<Vec3D> for Color {
    fn from(v: Vec3D) -> Self {
        Color::new(v.x(), v.y(), v.z())
    }
}
