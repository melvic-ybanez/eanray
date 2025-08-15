use crate::core::hittables::HitRecord;
use crate::core::math::interval::Interval;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::{hittables, Material, Ray};
use serde::{Deserialize, Serialize};
use crate::core::aabb::AABB;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Plane {
    p0: Point,    // a point on the plane
    n: UnitVec3D, // plane normal
    mat: Material,
    bbox: AABB
}

impl Plane {
    pub(crate) fn new(p0: Point, n: UnitVec3D, mat: Material) -> Self {
        Self { p0, n, mat, bbox: AABB::universe() }
    }

    /// A plane is defined as the set of all points `P` such that `dot(n, P - p0) = 0`.
    /// In other words, the plane's normal is perpendicular to all the vectors that lie on the plane.
    /// Given the formula above and the definition for ray (`P(t) = O + t * D`), if we solve for `t`, we'll get
    /// `t = dot(n, p0 - O) / dot(n, D)`.
    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denom = self.n.dot(ray.direction());

        // if the ray is not parallel to the plane
        if denom.abs() > 1e-6 {
            let t = self.n.dot(&(&self.p0 - ray.origin())) / denom;
            if ray_t.surrounds(t) {
                let hit_point = ray.at(t);
                let (front_face, face_normal) = HitRecord::face_normal(ray, self.n.clone());

                let (u_vec, v_vec) = self.compute_uv();
                let delta = &hit_point - &self.p0;

                Some(HitRecord::new(
                    hittables::P(hit_point),
                    hittables::Normal(face_normal),
                    hittables::Mat(&self.mat),
                    hittables::T(t),
                    hittables::FrontFace(front_face),
                    hittables::U(delta.dot(&u_vec)),
                    hittables::V(delta.dot(&v_vec)),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn compute_uv(&self) -> (Vec3D, Vec3D) {
        // If `n` is almost aligned with x, choose y. Otherwise, choose x.
        let temp = if self.n.x.abs() > 0.9 {
            Vec3D::new(0.0, 1.0, 0.0)
        } else {
            Vec3D::new(1.0, 0.0, 0.0)
        };
        let u = self.n.cross(&temp).to_unit();
        let v = self.n.cross(&u).to_unit();
        (u.0, v.0)
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
