use serde::{Deserialize, Serialize};
use crate::core::aabb::AABB;
use crate::core::hit::{HitRecord, ObjectRef};
use crate::core::{math, Ray};
use crate::core::math::{Axis, Point, Real, Vec3D};
use crate::core::math::interval::Interval;
use crate::core::math::vector::UnitVec3D;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RotateY {
    object: ObjectRef,
    sin_theta: Real,
    cos_theta: Real,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: ObjectRef, angle: Real) -> Self {
        let radians = math::degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point::from_scalar(math::INFINITY);
        let mut max = Point::from_scalar(-math::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = math::lerp(bbox.x().min, bbox.x().max, i as Real);
                    let y = math::lerp(bbox.y().min, bbox.y().max, j as Real);
                    let z = math::lerp(bbox.z().min, bbox.z().max, k as Real);

                    let (new_x, new_z) = Self::rotate(sin_theta, cos_theta, x, z);
                    let tester = Vec3D::new(new_x, y, new_z);

                    for c in 0..3 {
                        let c = Axis::from_usize_unsafe(c);
                        min[&c] = min[&c].min(tester[&c]);
                        max[&c] = max[&c].max(tester[&c]);
                    }
                }
            }
        }

        let bbox = AABB::from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let rotated_ray = self.to_object_space(ray);

        let mut hit_record = self.object.hit(&rotated_ray, ray_t)?;
        let (p, normal) = self.to_world_space(&mut hit_record);
        hit_record.p = p;
        hit_record.normal = normal;
        Some(hit_record)
    }

    /// We use the following formulas to transform the ray from world space to object space:
    ///  1. `x' = cos(theta) * x - sin(theta) * z`
    ///  2. `z' = sin(theta) * x + cos(theta) * z`
    /// since `cos(theta) = cos(-theta)` and `sin(-theta) = -sin(theta)`.
    /// In other words, we are rotating by `-theta` here.
    fn to_object_space(&self, ray: &Ray) -> Ray {
        let origin = ray.origin();
        let direction = ray.direction();

        let (x, z) = Self::rotate(-self.sin_theta, self.cos_theta, origin.x, origin.z);
        let origin = Point::new(x, origin.y, z);

        let (x, z) = Self::rotate(-self.sin_theta, self.cos_theta, direction.x, direction.z);
        let direction = Vec3D::new(x, direction.y, z);

        Ray::new_timed(origin, direction, ray.time())
    }

    /// This is the opposite of [[self.to_object_space]]. It uses the formula for rotating with theta.
    fn to_world_space(&self, record: &HitRecord) -> (Point, UnitVec3D) {
        let p = record.p();
        let normal = record.normal();

        let (x, z) = Self::rotate(self.sin_theta, self.cos_theta, p.x, p.z);
        let p = Point::new(x, p.y, z);

        let (x, z) = Self::rotate(self.sin_theta, self.cos_theta, normal.0.x, normal.0.z);
        let normal = UnitVec3D(Vec3D::new(x, normal.0.y, z));

        (p, normal)
    }

    /// Rotation around y requires the following formulas:
    ///     1. `x' = cos(theta) * x + sin(theta) * z`
    ///     2. `z' = -sin(theta) * x + cos(theta) * z`
    fn rotate(sin_theta: Real, cos_theta: Real, x: Real, z: Real) -> (Real, Real) {
        (
            cos_theta * x + sin_theta * z,
            -sin_theta * x + cos_theta * z,
        )
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}