use serde::{Deserialize, Serialize};
use crate::core::math::interval::Interval;
use crate::core::math::vector::{CanAdd, Vec3D, VecLike};
use crate::core::math::Real;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

        let to_byte = |component: Real| -> u16 {
            let component = Color::linear_to_gamma(component);
            (intensity.clamp(component) * 256.0) as u16
        };

        format!(
            "{} {} {}",
            to_byte(self.red_component()),
            to_byte(self.green_component()),
            to_byte(self.blue_component())
        )
    }

    fn linear_to_gamma(linear_component: Real) -> Real {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }
}

impl CanAdd for ColorKind {}

impl From<Vec3D> for Color {
    fn from(v: Vec3D) -> Self {
        Color::new(v.x, v.y, v.z)
    }
}
