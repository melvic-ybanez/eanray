use crate::core::hit::HitRecord;
use crate::core::math::{Real, Vec3D};
use crate::core::{Color, Ray};

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: Real },
    Dielectric { refraction_index: Real }
}

impl Material {
    pub fn scatter<'a>(&self, ray_in: &Ray<'a>, rec: &'a HitRecord) -> Option<(Ray<'a>, Color)> {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_direction = &rec.normal().0 + Vec3D::random_unit().0;
                let scatter_direction = if scatter_direction.near_zero() {
                    rec.normal().0.clone()
                } else {
                    scatter_direction
                };
                let scattered = Ray::new(rec.p(), scatter_direction);
                let attenuation = albedo.clone();
                Some((scattered, attenuation))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = ray_in.direction().reflect(&rec.normal());
                let reflected = reflected.to_unit().0 + Vec3D::random_unit().0 * *fuzz;
                let scattered = Ray::new(rec.p(), reflected);
                let attenuation = albedo.clone();
                Some((scattered, attenuation))
            }
            Material::Dielectric { refraction_index } => {
                let ri = if rec.front_face() {
                    1.0 / refraction_index
                } else {
                    *refraction_index
                };
                let unit_direction = ray_in.direction().to_unit();
                let refracted = Vec3D::refract(&unit_direction, rec.normal(), ri);
                let scattered = Ray::new(rec.p(), refracted);

                Some((scattered, Color::white()))
            }
        }
    }

    pub fn new_metal(albedo: Color, fuzz: Real) -> Material {
        Material::Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0} }
    }
}
