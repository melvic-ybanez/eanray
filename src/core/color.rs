use crate::core::math;
use crate::core::math::Real;
use crate::core::math::interval::Interval;
use crate::core::math::macros::impl_from_for_vec_like;
use crate::core::math::vector::{CanAdd, PointKind, VecKind, VecLike};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ColorKind;

pub(crate) type Color = VecLike<ColorKind>;

impl Color {
    pub(crate) fn red_component(&self) -> Real {
        self.x
    }

    pub(crate) fn green_component(&self) -> Real {
        self.y
    }

    pub(crate) fn blue_component(&self) -> Real {
        self.z
    }

    pub(crate) fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub(crate) fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub(crate) fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }

    pub(crate) fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }

    pub(crate) fn green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }

    pub(crate) fn cyan() -> Color {
        Color::new(0.0, 1.0, 1.0)
    }

    pub(crate) fn to_bytes_string(&self) -> String {
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
            linear_component.powf(1.0 / math::GAMMA as f64)
        } else {
            0.0
        }
    }
}

impl CanAdd for ColorKind {}

impl_from_for_vec_like!(VecKind, ColorKind);
impl_from_for_vec_like!(ColorKind, VecKind);
impl_from_for_vec_like!(PointKind, ColorKind);
impl_from_for_vec_like!(ColorKind, PointKind);
