use crate::core::aabb::AABB;
use crate::core::materials::Material;
use crate::core::math;
use crate::core::math::vector::{Point, UnitVec3D};
use crate::core::math::{Real, Vec3D};
use crate::core::ray::Ray;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Sphere {
    // we are using a Ray for the center as opposed to a Point to allow for "motion" effects
    center: Ray,

    radius: Real,
    mat: Material,
    bbox: AABB,
}

impl Sphere {
    /// Alias for [[Self::stationary]]. This makes the API more consistent with the rest of the shapes.
    /// Also, stationary is probably used a lot more than [[Self::moving]] so making it the sensible
    /// default might improve ergonomics.
    pub(crate) fn new(center: Point, radius: Real, mat: Material) -> Self {
        Self::stationary(center, radius, mat)
    }

    pub(crate) fn stationary(center: Point, radius: Real, mat: Material) -> Self {
        let r_vec = Vec3D::from_scalar(radius);
        let bbox = AABB::from_points(&center - &r_vec, &center + r_vec);
        Self::from_ray_props(center, Vec3D::zero(), radius, mat, bbox)
    }

    pub(crate) fn moving(center1: Point, center2: Point, radius: Real, mat: Material) -> Self {
        let r_vec = Vec3D::from_scalar(radius);
        let mut this = Self::from_ray_props(
            center1.clone(),
            center2 - center1,
            radius,
            mat.clone(),
            AABB::empty(),
        );

        let box1 = AABB::from_points(&this.center.at(0.0) - &r_vec, this.center.at(0.0) + &r_vec);
        let box2 = AABB::from_points(&this.center.at(1.0) - &r_vec, this.center.at(1.0) + &r_vec);

        this.bbox = AABB::from_boxes(&box1, &box2);

        this
    }

    fn from_ray_props(
        center: Point,
        direction: Vec3D,
        radius: Real,
        mat: Material,
        bbox: AABB,
    ) -> Self {
        Self {
            center: Ray::new(center, direction),
            radius: Real::max(0.0, radius),
            mat,
            bbox,
        }
    }

    pub(super) fn compute_uv(&self, p: &Vec3D) -> (Real, Real) {
        // NOTE: `p` should have been a Point by definition, but I'll allow a Vec
        // this time to avoid having to cast

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + math::PI;

        let u = phi / (2.0 * math::PI);
        let v = theta / math::PI;
        (u, v)
    }

    pub(super) fn computations(&self, ray: &Ray) -> (Real, Real, Real, Point) {
        let current_center = self.center.at(ray.time());
        let oc = &current_center - ray.origin();
        let a = ray.direction().dot(&ray.direction());
        let b = ray.direction().dot(&oc) * -2.0;
        let c = oc.length_squared() - self.radius * self.radius;
        (a, b, c, current_center)
    }

    pub(super) fn compute_outward_normal(&self, current_center: &Point, p: &Point) -> UnitVec3D {
        UnitVec3D((p - current_center) / self.radius)
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    pub(super) fn material(&self) -> &Material {
        &self.mat
    }
}
