use crate::core::aabb::AABB;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::{Material, Ray, math};
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

    pub(crate) fn finite(radius: Real, height: Real, material: Material) -> Self {
        Self::new(radius, material, CylinderKind::Finite { height })
    }

    /// The formula for an infinite vertical cylinder with radius `r` is `x^2 + z^2 = r^2`.
    /// If we plug in a ray `o + td`, we'd get: `(o_x + td_x)^2 + (o_z + td_z)^2 = r^2`.
    pub(super) fn computations(&self, ray: &Ray) -> Option<(Real, Real, Real, Point)> {
        // for brevity, and to mimic the variables above
        let d = ray.direction();
        let o = ray.origin();
        let r = self.radius;

        // Expanding the equation into quadratic form, and solving for a, b, and c, we have:
        let a = d.x * d.x + d.z * d.z;
        let b = 2.0 * (o.x * d.x + o.z * d.z);
        let c = o.x * o.x + o.z * o.z - r * r;

        if a.abs() < math::EPSILON {
            None
        } else {
            // For now, the current-center is set to (0, 0, 0). We might have to revisit this
            // once motion blurs for cylinders have been implemented
            Some((a, b, c, Point::zero()))
        }
    }

    pub(super) fn compute_outward_normal(&self, p: &Point) -> UnitVec3D {
        UnitVec3D(Vec3D::new(p.x, 0.0, p.z) / self.radius)
    }

    pub(super) fn check_y_hit(&self, ray: &Ray, root: Real) -> bool {
        match self.kind {
            CylinderKind::Finite { height } => {
                let y_hit = ray.origin().y + root * ray.direction().y;
                let half = height / 2.0;
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
            CylinderKind::Finite { height } => {
                let half = height / 2.0;
                (-half, half)
            }
            CylinderKind::Infinite => (-math::INFINITY, math::INFINITY),
        };

        self.bbox = AABB::from_points(
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        )
    }

    pub(super) fn compute_uv(&self, p: &Vec3D) -> (Real, Real) {
        let theta = p.z.atan2(p.x); // angle around the y-axis. range: (-pi, pi]
        let u = theta / (2.0 * math::PI) + 0.5; // convert to [0, 1].

        let v = match self.kind {
            CylinderKind::Finite { height } => (p.y + height / 2.0) / height, // normalize to [0, 1]
            CylinderKind::Infinite => p.y % 1.0,
        };

        (u, v)
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CylinderKind {
    Finite { height: Real },
    Infinite,
}
