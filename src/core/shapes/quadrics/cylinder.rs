use crate::core::aabb::AABB;
use crate::core::hittables::HitRecord;
use crate::core::math::interval::Interval;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::shapes::quadrics::cone::HitType;
use crate::core::shapes::quadrics::{cone, point_within_disk};
use crate::core::{math, Material, Ray};
use serde::{Deserialize, Serialize};

/// Theoretically, the finite cylinder can be encoded as a truncated cone with the same top
/// and bottom radii, so we can probably just unify both. However, the generalization
/// might make it more expensive to compute normals for cylinders, which is much simpler here.
/// (I haven't profiled this, so I don't know)
///
/// Let's keep them separate for now and maybe revisit these implementations in the future.
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

    pub(crate) fn closed(
        radius: Real,
        height: Real,
        side_mat: Material,
        cap_mat: Material,
    ) -> Self {
        Self::finite(radius, height, side_mat, FiniteType::Closed { cap_mat })
    }

    /// The formula for a vertical cylinder with radius `r` is `x^2 + z^2 = r^2`.
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
            cone::compute_root(a, b, c, ray_t, |root| self.check_y_hit(ray, root))
        };

        let (t, hit_type) = cone::nearest_hit(t0, self.nearest_cap_hit(ray, ray_t));
        let compute_mat = || match hit_type {
            HitType::Side => &self.mat,
            _ => match &self.kind {
                CylinderKind::Finite {
                    kind: FiniteType::Closed { cap_mat },
                    ..
                } => &cap_mat,
                _ => &self.mat,
            },
        };

        cone::record_hit_details(
            (t, hit_type.clone()),
            ray,
            |p| UnitVec3D(Vec3D::new(p.x, 0.0, p.z) / self.radius),
            |outward_normal| self.compute_uv(outward_normal, hit_type.clone()),
            compute_mat,
        )
    }

    fn check_y_hit(&self, ray: &Ray, t: Real) -> bool {
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
        let (min_y, max_y) = match self.kind {
            CylinderKind::Finite { half, .. } => (-half, half),
            CylinderKind::Infinite => (-math::INFINITY, math::INFINITY),
        };

        self.bbox = AABB::from_points(
            Point::new(-self.radius, min_y, -self.radius),
            Point::new(self.radius, max_y, self.radius),
        )
    }

    pub(super) fn compute_uv(&self, p: &Vec3D, hit_type: HitType) -> (Real, Real) {
        match hit_type {
            HitType::Side => {
                let v = match self.kind {
                    CylinderKind::Finite { half, .. } => (p.y + half) / (half * 2.0), // normalize to [0, 1]
                    CylinderKind::Infinite => p.y % 1.0,
                };

                (cone::compute_side_u(p), v)
            }
            HitType::ApexCap | HitType::BaseCap => cone::compute_cap_uv(p, self.radius),
        }
    }

    pub fn nearest_cap_hit(&self, ray: &Ray, ray_t: &Interval) -> (Real, HitType) {
        match self.kind {
            CylinderKind::Finite {
                half,
                kind: FiniteType::Closed { .. },
            } => cone::compare_cap_hits(
                self.hit_cap(ray, ray_t, half),
                self.hit_cap(ray, ray_t, -half),
            ),
            _ => (math::INFINITY, HitType::Side),
        }
    }

    fn hit_cap(&self, ray: &Ray, ray_t: &Interval, height: Real) -> Option<Real> {
        point_within_disk(ray, ray_t, height, self.radius)
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum FiniteType {
    Closed { cap_mat: Material },
    Open,
}
