use crate::core::hit::HitRecord;
use crate::core::math::Vec3D;
use crate::core::{Color, Ray};

pub enum Material {
    Lambertian { albedo: Color },
}

impl Material {
    fn scatter<'a>(&self, ray: &Ray<'a>, rec: &'a HitRecord) -> Option<(Ray<'a>, Color)> {
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
        }
    }
}
