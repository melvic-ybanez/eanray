use crate::core::hit::HitRecord;
use crate::core::math::{Real, Vec3D};
use crate::core::{math, Color, Ray};
use serde::{Deserialize, Serialize};
use crate::core::textures::{SolidColor, Texture};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter<'a>(&self, ray_in: &Ray<'a>, rec: &'a HitRecord) -> Option<(Ray<'a>, Color)> {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(ray_in, rec),
            Material::Metal(metal) => metal.scatter(ray_in, rec),
            Material::Dielectric(dielectric) => dielectric.scatter(ray_in, rec),
        }
    }

    pub fn new_lambertian(albedo: Color) -> Self {
        Material::Lambertian(Lambertian::from_albedo(albedo))
    }

    pub fn new_metal(albedo: Color, fuzz: Real) -> Self {
        Material::Metal(Metal::new(albedo, fuzz))
    }

    pub fn new_dielectric(refraction_index: Real) -> Self {
        Material::Dielectric(Dielectric::new(refraction_index))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lambertian {
    texture: Texture,
}

impl Lambertian {
    pub fn new(texture: Texture) -> Self {
        Self { texture }
    }

    pub fn from_albedo(albedo: Color) -> Self {
        Self::new(Texture::SolidColor(SolidColor::new(albedo)))
    }

    fn scatter<'a>(&self, ray_in: &Ray<'a>, rec: &'a HitRecord) -> Option<(Ray<'a>, Color)> {
        let scatter_direction = &rec.normal().0 + Vec3D::random_unit().0;
        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal().0.clone()
        } else {
            scatter_direction
        };
        let scattered = Ray::from_ref_origin_timed(rec.p(), scatter_direction, ray_in.time());
        let attenuation = self.texture.value(rec.u(), rec.v(), rec.p());
        Some((scattered, attenuation))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metal {
    albedo: Color,
    fuzz: Real,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Real) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    fn scatter<'a>(&self, ray_in: &Ray<'a>, rec: &'a HitRecord) -> Option<(Ray<'a>, Color)> {
        let reflected = ray_in.direction().reflect(&rec.normal());
        let reflected = reflected.to_unit().0 + Vec3D::random_unit().0 * self.fuzz;
        let scattered = Ray::from_ref_origin_timed(rec.p(), reflected, ray_in.time());
        let attenuation = self.albedo.clone();
        Some((scattered, attenuation))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dielectric {
    refraction_index: Real,
}

impl Dielectric {
    pub fn new(refraction_index: Real) -> Self {
        Self { refraction_index }
    }

    fn scatter<'a>(&self, ray_in: &Ray<'a>, rec: &'a HitRecord) -> Option<(Ray<'a>, Color)> {
        let ri = if rec.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction().to_unit();

        let cos_theta = (-&unit_direction.0).dot(&rec.normal().0).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > math::random_real() {
            unit_direction.0.reflect(rec.normal())
        } else {
            Vec3D::refract(&unit_direction, rec.normal(), ri)
        };

        let scattered = Ray::from_ref_origin_timed(rec.p(), direction, ray_in.time());

        Some((scattered, Color::white()))
    }

    /// Computes the reflectance using Schlick's Approximation
    fn reflectance(cosine: Real, refraction_index: Real) -> Real {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

pub mod refractive_index {
    use crate::core::math::Real;

    pub const GLASS: Real = 1.5;
    pub const VACUUM: Real = 1.0;
    pub const AIR: Real = 1.0003;
    pub const WATER: Real = 1.333;
    pub const DIAMOND: Real = 2.417;
}
