use crate::core::aabb::AABB;
use crate::core::hittables::HitRecord;
use crate::core::math::interval::Interval;
use crate::core::math::Real;
use crate::core::shapes::quadrics::cone::Cone;
use crate::core::shapes::Sphere;
use crate::core::Ray;
use cylinder::Cylinder;
use serde::{Deserialize, Serialize};

pub(crate) mod cone;
pub(crate) mod cylinder;
pub(crate) mod sphere;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum Quadric {
    Sphere(Sphere),
    Cylinder(Cylinder),
    Cone(Cone),
}

impl Quadric {
    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(sphere) => sphere.hit(ray, ray_t),
            Self::Cylinder(cylinder) => cylinder.hit(ray, ray_t),
            Self::Cone(cone) => cone.hit(ray, ray_t),
        }
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        match self {
            Self::Sphere(sphere) => sphere.bounding_box(),
            Self::Cylinder(cylinder) => cylinder.bounding_box(),
            Self::Cone(cone) => cone.bounding_box()
        }
    }
}

fn point_within_disk(ray: &Ray, ray_t: &Interval, height: Real, radius: Real) -> Option<Real> {
    if ray.direction().y == 0.0 {
        None
    } else {
        // since y = height and p(t) = origin + t * direction
        let t = (height - ray.origin().y) / ray.direction().y;

        let x = ray.origin().x + t * ray.direction().x;
        let z = ray.origin().z + t * ray.direction().z;

        if x * x + z * z <= radius * radius && ray_t.surrounds(t) {
            Some(t)
        } else {
            None
        }
    }
}

fn compute_root_from_discriminant<F>(
    discriminant: Real,
    a: Real,
    b: Real,
    ray_t: &Interval,
    check_range: F,
) -> Option<Real>
where
    F: Fn(Real) -> bool,
{
    let sqrt_d = discriminant.sqrt();
    let root = (-b - sqrt_d) / (2.0 * a);
    if !ray_t.surrounds(root) || !check_range(root) {
        let root = (-b + sqrt_d) / (2.0 * a);
        if !ray_t.surrounds(root) || !check_range(root) {
            None
        } else {
            Some(root)
        }
    } else {
        Some(root)
    }
}