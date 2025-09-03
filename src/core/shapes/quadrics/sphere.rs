use crate::core::aabb::AABB;
use crate::core::hittables::HitRecord;
use crate::core::materials::Material;
use crate::core::math::interval::Interval;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::math::ray::Ray;
use crate::core::{hittables, math};
use serde::{Deserialize, Serialize};
use crate::core::shapes::quadrics::compute_root_from_discriminant;

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

    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time());
        let oc = &current_center - ray.origin();
        let a = ray.direction().dot(&ray.direction());
        let b = ray.direction().dot(&oc) * -2.0;
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = math::discriminant(a, b, c);

        if discriminant < 0.0 {
            None
        } else {
           let root = compute_root_from_discriminant(discriminant, a, b, ray_t, |_| true);

            root.map(|root| {
                let p = ray.at(root);
                let outward_normal = UnitVec3D((&p - current_center) / self.radius);
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

    pub(super) fn compute_uv(&self, p: &Vec3D) -> (Real, Real) {
        // NOTE: `p` should have been a Point by definition, but I'll allow a Vec
        // this time to avoid having to cast

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + math::PI;

        let u = phi / (2.0 * math::PI);
        let v = theta / math::PI;
        (u, v)
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    pub(super) fn material(&self) -> &Material {
        &self.mat
    }
}
