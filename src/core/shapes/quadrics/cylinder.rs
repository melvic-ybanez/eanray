use crate::core::aabb::AABB;
use crate::core::hittables::HitRecord;
use crate::core::math::interval::Interval;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::{hittables, math, Color, Material, Ray};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Cylinder {
    radius: Real,
    mat: Material,
    bbox: AABB,
    pub(super) kind: CylinderKind,
}

impl Cylinder {
    fn new(radius: Real, material: Material, kind: CylinderKind) -> Self {
        let mut this = Self {
            radius,
            mat: material,
            bbox: AABB::empty(),
            kind,
        };
        this.compute_bounding_box();
        this
    }

    pub(crate) fn infinite(radius: Real, material: Material) -> Self {
        Self::new(radius, material, CylinderKind::Infinite)
    }

    pub(crate) fn finite(radius: Real, height: Real, material: Material, kind: FiniteType) -> Self {
        Self::new(
            radius,
            material,
            CylinderKind::Finite {
                half: height / 2.0,
                kind,
            },
        )
    }

    pub(crate) fn open(radius: Real, height: Real, material: Material) -> Self {
        Self::finite(radius, height, material, FiniteType::Open)
    }

    pub(crate) fn closed(radius: Real, height: Real, side_mat: Material, cap_mat: Material) -> Self {
        Self::finite(radius, height, side_mat, FiniteType::Closed { cap_mat })
    }

    /// The formula for an infinite vertical cylinder with radius `r` is `x^2 + z^2 = r^2`.
    /// If we plug in a ray `o + td`, we'd get: `(o_x + td_x)^2 + (o_z + td_z)^2 = r^2`.
    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // for brevity, and to mimic the variables above
        let d = ray.direction();
        let o = ray.origin();
        let r = self.radius;

        // Expanding the equation into quadratic form, and solving for a, b, and c, we have:
        let a = d.x * d.x + d.z * d.z;
        let b = 2.0 * (o.x * d.x + o.z * d.z);
        let c = o.x * o.x + o.z * o.z - r * r;

        let t0 = if a.abs() < math::EPSILON {
            // the ray is parallel to the y-axis
            math::INFINITY
        } else {
            let discriminant = b * b - 4.0 * a * c;

            if discriminant < 0.0 {
                math::INFINITY
            } else {
                let sqrt_d = discriminant.sqrt();
                let root = (-b - sqrt_d) / (2.0 * a);
                if !ray_t.surrounds(root) || !self.check_y_hit(ray, root) {
                    let root = (-b + sqrt_d) / (2.0 * a);
                    if !ray_t.surrounds(root) || !self.check_y_hit(ray, root) {
                        math::INFINITY
                    } else {
                        root
                    }
                } else {
                    root
                }
            }
        };

        let (t1, hit_type) = self.nearest_cap_hit(ray, ray_t);
        let (t, hit_type) = if t0 < t1 { (t0, HitType::Side) } else { (t1, hit_type) };
        if t == math::INFINITY {
            None
        } else {
            let p = ray.at(t);
            let outward_normal = match hit_type {
                HitType::Side => UnitVec3D(Vec3D::new(p.x, 0.0, p.z) / self.radius),
                HitType::TopCap => Vec3D::new(0.0, 1.0, 0.0).to_unit(),
                HitType::BottomCap => Vec3D::new(0.0, -1.0, 0.0).to_unit(),
            };
            let (u, v) = self.compute_uv(&outward_normal, hit_type);
            let (front_face, face_normal) = HitRecord::face_normal(&ray, outward_normal);

            Some(HitRecord::new(
                hittables::P(p),
                hittables::Normal(face_normal),
                hittables::Mat(self.material()),
                hittables::T(t),
                hittables::FrontFace(front_face),
                hittables::U(u),
                hittables::V(v),
            ))
        }
    }

    pub(super) fn check_y_hit(&self, ray: &Ray, t: Real) -> bool {
        match self.kind {
            CylinderKind::Finite { half, .. } => {
                let y_hit = ray.origin().y + t * ray.direction().y;
                -half <= y_hit && y_hit <= half
            }
            _ => true,
        }
    }

    pub(crate) fn material(&self) -> &Material {
        &self.mat
    }

    fn compute_bounding_box(&mut self) {
        let min_x = -self.radius;
        let min_z = -self.radius;
        let max_x = self.radius;
        let max_z = self.radius;

        let (min_y, max_y) = match self.kind {
            CylinderKind::Finite { half, .. } => (-half, half),
            CylinderKind::Infinite => (-math::INFINITY, math::INFINITY),
        };

        self.bbox = AABB::from_points(
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        )
    }

    pub(super) fn compute_uv(&self, p: &Vec3D, hit_type: HitType) -> (Real, Real) {
        match hit_type {
            HitType::Side => {
                let theta = p.z.atan2(p.x); // angle around the y-axis. range: (-pi, pi]
                let u = theta / (2.0 * math::PI) + 0.5; // convert to [0, 1].

                let v = match self.kind {
                    CylinderKind::Finite { half, .. } => (p.y + half) / (half * 2.0), // normalize to [0, 1]
                    CylinderKind::Infinite => p.y % 1.0,
                };

                (u, v)
            }
            HitType::TopCap | HitType::BottomCap => {
                // Treat the caps like flat disks, so x and z are elements of [-r, r].
                // Then normalize.
                let u = p.x / (2.0 * self.radius) + 0.5;
                let v = p.z / (2.0 * self.radius) + 0.5;
                (u, v)
            }
        }
    }

    pub(crate) fn nearest_cap_hit(&self, ray: &Ray, ray_t: &Interval) -> (Real, HitType) {
        match self.kind {
            CylinderKind::Finite { half, kind: FiniteType::Closed {..} } => {
                match (self.hit_cap(ray, ray_t, half), self.hit_cap(ray, ray_t, -half)) {
                    (Some(t1), Some(t2)) => {
                        if t1 < t2 {
                            (t1, HitType::TopCap)
                        } else {
                            (t2, HitType::BottomCap)
                        }
                    }
                    (Some(t1), _) => (t1, HitType::TopCap),
                    (_, Some(t2)) => (t2, HitType::BottomCap),
                    _ => (math::INFINITY, HitType::Side),
                }
            }
            _ => (math::INFINITY, HitType::Side)
        }
    }

    fn hit_cap(&self, ray: &Ray, ray_t: &Interval, height: Real) -> Option<Real> {
        if ray.direction().y == 0.0 {
            None
        } else {
            // since y = height and p(t) = origin + t * direction
            let t = (height - ray.origin().y) / ray.direction().y;

            let x = ray.origin().x + t * ray.direction().x;
            let z = ray.origin().z + t * ray.direction().z;

            if x * x + z * z <= self.radius * self.radius && ray_t.surrounds(t) {
                Some(t)
            } else {
                None
            }
        }
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum CylinderKind {
    Finite { half: Real, kind: FiniteType },
    Infinite,
}

pub(crate) enum HitType {
    Side,
    TopCap,
    BottomCap,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum FiniteType {
    Closed { cap_mat: Material },
    Open,
}
