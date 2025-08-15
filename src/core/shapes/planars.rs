use crate::core::aabb::AABB;
use crate::core::hittables::HitRecord;
use crate::core::math::interval::Interval;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::{Material, Ray, hittables};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Kind {
    Quad(Quad),
    Triangle(Triangle),
    Disk(Disk),
}

/// Represents any 2D planar primitive. I'm under the impression that `q`, `u`, and `v` are standard
/// names in ray tracing, so I'll use them here as well.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Planar {
    q: Point,
    u: Vec3D,
    v: Vec3D,
    mat: Material,
    bbox: AABB,
    normal: UnitVec3D,
    d: Real, // this is the D in Ax + By + Cz = D
    w: Vec3D,
    kind: Kind,
}

impl Planar {
    pub fn quad(q: Point, u: Vec3D, v: Vec3D, mat: Material) -> Self {
        Self::new(q, u, v, mat, Kind::Quad(Quad))
    }

    pub fn triangle(q: Point, u: Vec3D, v: Vec3D, mat: Material) -> Self {
        Self::new(q, u, v, mat, Kind::Triangle(Triangle))
    }

    pub fn disk(q: Point, u: Vec3D, v: Vec3D, radius: Real, mat: Material) -> Self {
        Self::new(q, u, v, mat, Kind::Disk(Disk { radius }))
    }

    pub fn new(q: Point, u: Vec3D, v: Vec3D, mat: Material, kind: Kind) -> Self {
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
            kind,
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

        if self.is_interior(alpha, beta) {
            let (front_face, face_normal) = HitRecord::face_normal(&ray, self.normal.clone());

            Some(HitRecord::new(
                hittables::P(intersection),
                hittables::Normal(face_normal),
                hittables::Mat(&self.mat),
                hittables::T(t),
                hittables::FrontFace(front_face),
                hittables::U(alpha),
                hittables::V(beta),
            ))
        } else {
            None
        }
    }

    fn is_interior(&self, a: Real, b: Real) -> bool {
        match &self.kind {
            Kind::Quad(_) => Quad::is_interior(a, b),
            Kind::Triangle(_) => Triangle::is_interior(a, b),
            Kind::Disk(disk) => disk.is_interior(a, b),
        }
    }
}

/// Quadrilaterals. This is technically a parallelogram, but for some reason,
/// Peter Shirley named it quad in his books, and I intend to adapt that name here.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quad;

impl Quad {
    fn is_interior(a: Real, b: Real) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        unit_interval.contains(a) && unit_interval.contains(b)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Triangle;

impl Triangle {
    fn is_interior(a: Real, b: Real) -> bool {
        a > 0.0 && b > 0.0 && a + b < 1.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Disk {
    radius: Real,
}

impl Disk {
    fn is_interior(&self, a: Real, b: Real) -> bool {
        (a * a + b * b).sqrt() < self.radius
    }
}
