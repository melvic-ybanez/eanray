use crate::core::hittables::HitRecord;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::textures::{SolidColor, Texture};
use crate::core::{math, Color, Ray};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
}

impl Material {
    pub(crate) fn scatter<'a, 'b, 'c>(
        &self,
        ray_in: &Ray,
        rec: &'b HitRecord,
    ) -> Option<(Ray, Color)> {
        match self {
            Self::Lambertian(lambertian) => Some(lambertian.scatter(ray_in, rec)),
            Self::Metal(metal) => Some(metal.scatter(ray_in, rec)),
            Self::Dielectric(dielectric) => Some(dielectric.scatter(ray_in, rec)),
            Self::Isotropic(isotropic) => Some(isotropic.scatter(ray_in, rec)),
            Self::DiffuseLight(_) => None,
        }
    }

    pub(crate) fn emitted(&self, u: Real, v: Real, p: &Point) -> Color {
        match self {
            Self::DiffuseLight(diffuse_light) => diffuse_light.emitted(u, v, p),
            _ => Color::black(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Lambertian {
    texture: Texture,
}

impl Lambertian {
    // TODO: Rename this to `from_texture`
    pub(crate) fn new(texture: Texture) -> Self {
        Self { texture }
    }

    pub(crate) fn from_albedo(albedo: Color) -> Self {
        Self::new(Texture::SolidColor(SolidColor::new(albedo)))
    }

    fn scatter<'a>(&self, ray_in: &Ray, rec: &'a HitRecord) -> (Ray, Color) {
        let scatter_direction = &rec.normal().0 + Vec3D::random_unit().0;
        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal().0.clone()
        } else {
            scatter_direction
        };
        let scattered = Ray::new_timed(rec.p().clone(), scatter_direction, ray_in.time());
        let attenuation = self.texture.value(rec.u(), rec.v(), rec.p());
        (scattered, attenuation)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Metal {
    albedo: Color,
    fuzz: Real,
}

impl Metal {
    pub(crate) fn new(albedo: Color, fuzz: Real) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    fn scatter<'a>(&self, ray_in: &Ray, rec: &'a HitRecord) -> (Ray, Color) {
        let reflected = ray_in.direction().reflect(&rec.normal());
        let reflected = reflected.to_unit().0 + Vec3D::random_unit().0 * self.fuzz;
        let scattered = Ray::new_timed(rec.p().clone(), reflected, ray_in.time());
        let attenuation = self.albedo.clone();
        (scattered, attenuation)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Dielectric {
    refraction_index: Real,
}

impl Dielectric {
    pub(crate) fn new(refraction_index: Real) -> Self {
        Self { refraction_index }
    }

    fn scatter<'a>(&self, ray_in: &Ray, rec: &'a HitRecord) -> (Ray, Color) {
        let ri = if rec.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction().to_unit();

        let cos_theta = (-&unit_direction.0).dot(&rec.normal().0).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > math::random_real()
        {
            unit_direction.reflect(rec.normal())
        } else {
            Vec3D::refract(&unit_direction, rec.normal(), ri)
        };

        let scattered = Ray::new_timed(rec.p().clone(), direction, ray_in.time());

        (scattered, Color::white())
    }

    /// Computes the reflectance using Schlick's Approximation
    fn reflectance(cosine: Real, refraction_index: Real) -> Real {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct DiffuseLight {
    texture: Texture,
    intensity: Color,
}

impl DiffuseLight {
    pub(crate) fn from_texture(texture: Texture) -> Self {
        Self::from_texture_intensified(texture, Self::default_intensity())
    }

    pub(crate) fn from_emission(emission_color: Color) -> Self {
        Self::from_emission_intensified(emission_color, Self::default_intensity())
    }

    pub(crate) fn from_texture_intensified(texture: Texture, intensity: Color) -> Self {
        Self { texture, intensity }
    }

    pub(crate) fn from_emission_intensified(emission_color: Color, intensity: Color) -> Self {
        Self::from_texture_intensified(
            Texture::SolidColor(SolidColor::new(emission_color)),
            intensity,
        )
    }

    pub(crate) fn emitted(&self, u: Real, v: Real, p: &Point) -> Color {
        self.texture.value(u, v, p) * self.intensity.clone()
    }

    pub(crate) fn default_intensity() -> Color {
        Color::white()
    }
}

pub(crate) mod refractive_index {
    use crate::core::math::Real;

    pub(crate) const GLASS: Real = 1.5;
    pub(crate) const VACUUM: Real = 1.0;
    pub(crate) const AIR: Real = 1.0003;
    pub(crate) const WATER: Real = 1.333;
    pub(crate) const DIAMOND: Real = 2.417;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Isotropic {
    texture: Texture,
}

impl Isotropic {
    pub(crate) fn from_albedo(albedo: Color) -> Self {
        Self::from_texture(Texture::SolidColor(SolidColor::new(albedo)))
    }

    pub(crate) fn from_texture(texture: Texture) -> Self {
        Self { texture }
    }

    pub(crate) fn scatter<'a>(&self, ray_in: &Ray, hit_record: &'a HitRecord) -> (Ray, Color) {
        let scattered = Ray::new_timed(
            hit_record.p().clone(),
            Vec3D::random_unit().0,
            ray_in.time(),
        );
        let attenuation = self
            .texture
            .value(hit_record.u(), hit_record.v(), hit_record.p());
        (scattered, attenuation)
    }
}
