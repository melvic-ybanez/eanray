use crate::core::aabb::AABB;
use crate::core::hittables::HitRecord;
use crate::core::math::interval::Interval;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::shapes::Sphere;
use crate::core::{Material, Ray, hittables};
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

        let (a, b, c, current_center) = self.computations(ray)?;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let root = (-b - sqrtd) / (2.0 * a);
            let root = if !ray_t.surrounds(root) || !self.check_root(root, ray) {
                let root = (-b + sqrtd) / (2.0 * a);
                if !ray_t.surrounds(root) || !self.check_root(root, ray) {
                    None
                } else {
                    Some(root)
                }
            } else {
                Some(root)
            };

            root.map(|root| {
                let p = ray.at(root);
                let outward_normal = self.compute_outward_normal(&current_center, &p);
                let (u, v) = self.compute_uv(&outward_normal);
                let (front_face, face_normal) = HitRecord::face_normal(&ray, outward_normal);

                HitRecord::new(
                    hittables::P(p),
                    hittables::Normal(face_normal),
                    hittables::Mat(self.material()),
                    hittables::T(root),
                    hittables::FrontFace(front_face),
                    hittables::U(u),
                    hittables::V(v),
                )
            })
        }
    }

    fn computations(&self, ray: &Ray) -> Option<(Real, Real, Real, Point)> {
        match self {
            Self::Sphere(sphere) => Some(sphere.computations(ray)),
            Self::Cylinder(cylinder) => cylinder.computations(ray),
        }
    }

    fn compute_outward_normal(&self, center: &Point, p: &Point) -> UnitVec3D {
        match self {
            Self::Sphere(sphere) => sphere.compute_outward_normal(center, p),
            Self::Cylinder(cylinder) => cylinder.compute_outward_normal(p),
        }
    }

    fn compute_uv(&self, p: &Vec3D) -> (Real, Real) {
        match self {
            Self::Sphere(sphere) => sphere.compute_uv(p),
            Self::Cylinder(cylinder) => cylinder.compute_uv(p),
        }
    }

    fn material(&self) -> &Material {
        match self {
            Self::Sphere(sphere) => sphere.material(),
            Self::Cylinder(cylinder) => cylinder.material(),
        }
    }

    fn check_root(&self, root: Real, ray: &Ray) -> bool {
        match self {
            Self::Sphere(_) => true,
            Self::Cylinder(cylinder) => cylinder.check_y_hit(ray, root),
        }
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        match self {
            Self::Sphere(sphere) => sphere.bounding_box(),
            Self::Cylinder(cylinder) => cylinder.bounding_box(),
        }
    }
}
