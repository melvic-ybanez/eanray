use crate::core::aabb::AABB;
use crate::core::hittables::HitRecord;
use crate::core::math::interval::Interval;
use crate::core::shapes::Sphere;
use crate::core::Ray;
use crate::diagnostics::metrics;
use cylinder::Cylinder;
use serde::{Deserialize, Serialize};

pub(crate) mod cylinder;
pub(crate) mod sphere;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum Quadric {
    Sphere(Sphere),
    Cylinder(Cylinder),
}

impl Quadric {
    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        metrics::increment_object_hit_attempt_count();

        match self {
            Self::Sphere(sphere) => sphere.hit(ray, ray_t),
            Self::Cylinder(cylinder) => cylinder.hit(ray, ray_t),
        }
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        match self {
            Self::Sphere(sphere) => sphere.bounding_box(),
            Self::Cylinder(cylinder) => cylinder.bounding_box(),
        }
    }
}
