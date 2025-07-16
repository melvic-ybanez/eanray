use crate::core::math::interval::Interval;
use crate::core::math::{Point, Real};
use crate::core::perlin::Perlin;
use crate::core::Color;
use image::{ImageReader, ImageResult, RgbImage};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Texture {
    SolidColor(SolidColor),
    Checker(Checker),
    Image(ImageTexture),
    Noise(NoiseTexture),
}

impl Texture {
    pub fn value(&self, u: Real, v: Real, p: &Point) -> Color {
        match self {
            Texture::SolidColor(solid) => solid.value().clone(),
            Texture::Checker(checker) => checker.value(u, v, p),
            Texture::Image(image) => image.value(u, v, p),
            Texture::Noise(noise) => noise.value(u, v, p),
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageTexture {
    image: SerializeableImage,
}

impl ImageTexture {
    pub fn from_path(filepath: &str) -> ImageResult<Self> {
        let _image = ImageReader::open(filepath)?;
        Ok(Self {
            image: _image.decode()?.to_rgb8().into(),
        })
    }

    pub fn from_path_unsafe(filepath: &str) -> Self {
        Self::from_path(filepath)
            .map_err(|err| log::error!("Error loading image: {:?}", err))
            .unwrap()
    }

    pub fn value(&self, u: Real, v: Real, p: &Point) -> Color {
        if self.image.height <= 0 {
            Color::cyan()
        } else {
            let interval = Interval::new(0.0, 1.0);
            let u = interval.clamp(u);
            let v = 1.0 - interval.clamp(v); // flip

            let i = (u * self.image.width as f64) as u32;
            let j = (v * self.image.height as f64) as u32;
            let pixel = self.image.get_pixel(i, j);

            let color_scale = 1.0 / 255.0;
            Color::new(
                Self::linear_color(color_scale * pixel[0] as f64),
                Self::linear_color(color_scale * pixel[1] as f64),
                Self::linear_color(color_scale * pixel[2] as f64),
            )
        }
    }

    /// Gamma correction (gamma = 1)
    fn linear_color(value: f64) -> f64 {
        value.powf(2.2)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerializeableImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl SerializeableImage {
    fn get_pixel(&self, x: u32, y: u32) -> [u8; 3] {
        let x = Self::clamp(x, 0, self.width);
        let y = Self::clamp(y, 0, self.height);

        // multiply by 3 because each pixel occupies 3 bytes for rgb
        let index = ((y * self.width + x) * 3) as usize;

        [self.data[index], self.data[index + 1], self.data[index + 2]]
    }

    fn clamp(x: u32, low: u32, high: u32) -> u32 {
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }
}

impl From<RgbImage> for SerializeableImage {
    fn from(value: RgbImage) -> Self {
        let (width, height) = value.dimensions();
        Self {
            width,
            height,
            data: value.into_raw(),
        }
    }
}

impl From<SerializeableImage> for RgbImage {
    fn from(value: SerializeableImage) -> Self {
        if let Some(img) = RgbImage::from_raw(value.width, value.height, value.data) {
            img
        } else {
            log::error!("Unable to create image from raw");
            RgbImage::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
    base_color: Color,
}

impl NoiseTexture {
    pub fn new(scale: f64, base_color: Color) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
            base_color
        }
    }

    fn value(&self, u: Real, v: Real, p: &Point) -> Color {
        let noise = (self.scale * p.z + 10.0 * self.noise.turbulence(p, 7)).sin();
        let brightness = 0.85 + 0.15 * noise; // 0.85 - 1.0
        &self.base_color * brightness
    }
}
