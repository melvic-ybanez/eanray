use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::core::math::{Point, Real};
use crate::core::Color;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Texture {
    SolidColor(SolidColor),
    Checker(Checker),
}

impl Texture {
    pub fn value(&self, u: Real, v: Real, p: &Point) -> Color {
        match self {
            Texture::SolidColor(solid) => solid.value().clone(),
            Texture::Checker(checker) => checker.value(u, v, p),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self::new(Color::new(red, green, blue))
    }

    pub fn value(&self) -> &Color {
        &self.albedo
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checker {
    scale_inverse: Real,
    even: Rc<Texture>,
    odd: Rc<Texture>,
}

impl Checker {
    pub fn new(scale: Real, even: Texture, odd: Texture) -> Self {
        Self {
            scale_inverse: 1.0 / scale,
            even: Rc::new(even),
            odd: Rc::new(odd),
        }
    }

    pub fn from_colors(scale: Real, c1: Color, c2: Color) -> Self {
        Self::new(
            scale,
            Texture::SolidColor(SolidColor::new(c1)),
            Texture::SolidColor(SolidColor::new(c2)),
        )
    }

    pub fn value(&self, u: Real, v: Real, p: &Point) -> Color {
        let x = (self.scale_inverse * p.x).floor() as i32;
        let y = (self.scale_inverse * p.y).floor() as i32;
        let z = (self.scale_inverse * p.z).floor() as i32;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
