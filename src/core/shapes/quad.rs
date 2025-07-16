use serde::{Deserialize, Serialize};
use crate::core::aabb::AABB;
use crate::core::hit::HitRecord;
use crate::core::math::interval::Interval;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::{hit, Material, Ray};

/// Quadrilaterals. This is technically a parallelogram, but for some reason,
/// Peter Shirley named it quad in his books, and I intend to adapt that name here. Also,
/// I'm under the impression that `q`, `u`, and `v` are standard names in ray
/// tracing, so I'll use them here as well.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quad {
    q: Point,
    u: Vec3D,
    v: Vec3D,
    mat: Material,
    bbox: AABB,
    normal: UnitVec3D,
    d: Real, // this is the D in Ax + By + Cz = D
    w: Vec3D,
}

impl Quad {
    pub fn new(q: Point, u: Vec3D, v: Vec3D, mat: Material) -> Self {
        let n = u.cross(&v);
        let normal = n.to_unit();
        let d = normal.0.dot(&q.clone().into());
        let w = &n / n.dot(&n);
        let mut this = Self {
            q,
            u,
            v,
            mat,
            bbox: AABB::empty(),
            normal,
            d,
            w,
        };
        this.compute_bounding_box();
        this
    }

    fn compute_bounding_box(&mut self) {
        let bbox_diagonal1 = AABB::from_points(self.q.clone(), &self.q + &self.u + &self.v);
        let bbox_diagonal2 = AABB::from_points(&self.q + &self.u, &self.q + &self.v);
        self.bbox = AABB::from_boxes(&bbox_diagonal1, &bbox_diagonal2);
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denom = self.normal.0.dot(ray.direction());

        // ray is parallel to the plane
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.0.dot(&ray.origin().into())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hit_point_vector = &intersection - &self.q;
        let alpha = self.w.dot(&planar_hit_point_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hit_point_vector));

        self.is_interior(alpha, beta).map(|(u, v)| {
            let (front_face, face_normal) = HitRecord::face_normal(&ray, self.normal.clone());

            HitRecord::new(
                hit::P(intersection),
                hit::Normal(face_normal),
                hit::Mat(&self.mat),
                hit::T(t),
                hit::FrontFace(front_face),
                hit::U(u),
                hit::V(v),
            )
        })
    }

    fn is_interior(&self, a: Real, b: Real) -> Option<(Real, Real)> {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            None
        } else {
            Some((a, b))
        }
    }
}
