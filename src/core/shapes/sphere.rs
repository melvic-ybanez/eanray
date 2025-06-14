use std::borrow::Cow;
use crate::core::hit::{self, HitRecord};
use crate::core::materials::Material;
use crate::core::math::interval::Interval;
use crate::core::math::vector::{Point, UnitVec3D};
use crate::core::math::{Real, Vec3D};
use crate::core::ray::Ray;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sphere<'a> {
    center: Ray<'a>,    // this is to allow for "motion" effects
    radius: Real,
    mat: Material,
}

impl<'a> Sphere<'a> {
    pub fn stationary(center: Point, radius: Real, mat: Material) -> Self {
        Self::from_ray_props(center, Vec3D::zero(), radius, mat)
    }

    pub fn moving(center1: Point, center2: Point, radius: Real, mat: Material) -> Self {
        Self::from_ray_props(center1.clone(), center2 - center1, radius, mat)
    }

    fn from_ray_props(center: Point, direction: Vec3D, radius: Real, mat: Material) -> Self {
        Self {
            center: Ray::new(Cow::Owned(center), direction),
            radius: Real::max(0.0, radius),
            mat,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time());
        let oc = &current_center - ray.origin();
        let a = ray.direction().dot(&ray.direction());
        let b = ray.direction().dot(&oc) * -2.0;
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let root = (-b - sqrtd) / (2.0 * a);
            let root = if !ray_t.surrounds(root) {
                let root = (-b + sqrtd) / (2.0 * a);
                if !ray_t.surrounds(root) {
                    None
                } else {
                    Some(root)
                }
            } else {
                Some(root)
            };
            root.map(|root| {
                let p = ray.at(root);
                let outward_normal = UnitVec3D((&p - current_center) / self.radius);
                let (front_face, face_normal) = HitRecord::face_normal(&ray, outward_normal);
                HitRecord::new(
                    hit::P(p),
                    hit::Normal(face_normal),
                    hit::Mat(&self.mat),
                    hit::T(root),
                    hit::FrontFace(front_face),
                )
            })
        }
    }
}
