use crate::core::hit::HitRecord;
use crate::core::math::{Real, Vec3D};
use crate::core::{Color, Ray};

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter<'a>(&self, ray_in: &Ray<'a>, rec: &'a HitRecord) -> Option<(Ray<'a>, Color)> {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(rec),
            Material::Metal(metal) => metal.scatter(ray_in, rec),
            Material::Dielectric(dielectric) => dielectric.scatter(ray_in, rec),
        }
    }
    
    pub fn new_lambertian(albedo: Color) -> Self {
        Material::Lambertian(Lambertian::new(albedo))
    }
    
    pub fn new_metal(albedo: Color, fuzz: Real) -> Self {
        Material::Metal(Metal::new(albedo, fuzz))
    }
    
    pub fn new_dielectric(refraction_index: Real) -> Self {
        Material::Dielectric(Dielectric::new(refraction_index))
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    fn scatter<'a>(&self, rec: &'a HitRecord) -> Option<(Ray<'a>, Color)> {
        let scatter_direction = &rec.normal().0 + Vec3D::random_unit().0;
        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal().0.clone()
        } else {
            scatter_direction
        };
        let scattered = Ray::new(rec.p(), scatter_direction);
        let attenuation = self.albedo.clone();
        Some((scattered, attenuation))
    }
}

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
        let scattered = Ray::new(rec.p(), reflected);
        let attenuation = self.albedo.clone();
        Some((scattered, attenuation))
    }
}

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
        let refracted = Vec3D::refract(&unit_direction, rec.normal(), ri);
        let scattered = Ray::new(rec.p(), refracted);

        Some((scattered, Color::white()))
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
