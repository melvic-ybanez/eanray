use crate::core::aabb::AABB;
use crate::core::hittables::{FrontFace, HitRecord, Mat, Normal, ObjectRef, P, T, U, V};
use crate::core::materials::Isotropic;
use crate::core::math::interval::Interval;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Real, Vec3D};
use crate::core::textures::Texture;
use crate::core::{Color, Material, Ray, math};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ConstantMedium {
    boundary: ObjectRef,
    neg_inv_density: Real,
    phase_function: Material,
}

impl ConstantMedium {
    pub(crate) fn new(boundary: ObjectRef, density: Real, phase_function: Material) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }

    pub(crate) fn from_texture(boundary: ObjectRef, density: Real, texture: Texture) -> Self {
        Self::new(
            boundary,
            density,
            Material::Isotropic(Isotropic::from_texture(texture)),
        )
    }

    pub(crate) fn from_albedo(boundary: ObjectRef, density: Real, albedo: Color) -> Self {
        Self::new(
            boundary,
            density,
            Material::Isotropic(Isotropic::from_albedo(albedo)),
        )
    }

    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec1 = self.boundary.hit(ray, &Interval::universe())?;
        let mut rec2 = self
            .boundary
            .hit(ray, &Interval::new(rec1.t + 0.0001, math::INFINITY))?;

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max
        }

        if rec1.t >= rec2.t {
            None
        } else {
            if rec1.t < 0.0 {
                rec1.t = 0.0
            }

            let ray_length = ray.direction().length();
            let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
            let hit_distance = self.neg_inv_density * math::random_real().log(std::f64::consts::E);

            if hit_distance > distance_inside_boundary {
                None
            } else {
                let t = rec1.t + hit_distance / ray_length;
                Some(HitRecord::new(
                    P(ray.at(t)),
                    Normal(UnitVec3D(Vec3D::new(1.0, 0.0, 0.0))),
                    Mat(&self.phase_function),
                    T(t),
                    FrontFace(true),
                    U(0.0),
                    V(0.0),
                ))
            }
        }
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        self.boundary.bounding_box()
    }
}
