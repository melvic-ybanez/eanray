use crate::core::aabb::AABB;
use crate::core::hittables::{HitRecord, ObjectRef};
use crate::core::math::interval::Interval;
use crate::core::math::vector::{UnitVec3D, VecKind};
use crate::core::math::{Axis, Point, Real, Vec3D, VecLike};
use crate::core::{Ray, math};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Translate {
    object: ObjectRef,
    offset: Vec3D,
    bbox: AABB,
}

impl Translate {
    pub(crate) fn new(object: ObjectRef, offset: Vec3D) -> Self {
        let bbox = object.bounding_box() + &offset;
        Self {
            object,
            offset,
            bbox,
        }
    }

    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let offset_origin = ray.origin() - &self.offset;
        let offset_ray = Ray::new_timed(offset_origin, ray.direction().clone(), ray.time());
        let mut hit_record = self.object.hit(&offset_ray, ray_t)?;
        hit_record.p = hit_record.p + &self.offset;
        Some(hit_record)
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum RotateKind {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Rotate {
    object: ObjectRef,
    sin_theta: Real,
    cos_theta: Real,
    bbox: AABB,
    kind: RotateKind,
}

impl Rotate {
    pub(crate) fn new(object: ObjectRef, angle: Real, kind: RotateKind) -> Self {
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

                    let tester = Self::rotate::<VecKind>(&kind, sin_theta, cos_theta, x, y, z);

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
            kind,
        }
    }

    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let rotated_ray = self.to_object_space(ray);

        let mut hit_record = self.object.hit(&rotated_ray, ray_t)?;
        let (p, normal) = self.to_world_space(&mut hit_record);
        hit_record.p = p;
        hit_record.normal = normal;
        Some(hit_record)
    }

    /// Transforms the ray from world space to object space.
    /// In other words, we are rotating by `-theta` here.
    /// Note: `cos(theta) = cos(-theta)` and `sin(-theta) = -sin(theta)`.
    fn to_object_space(&self, ray: &Ray) -> Ray {
        let origin = ray.origin();
        let direction = ray.direction();

        let origin = Self::rotate(
            &self.kind,
            -self.sin_theta,
            self.cos_theta,
            origin.x,
            origin.y,
            origin.z,
        );

        let direction = Self::rotate(
            &self.kind,
            -self.sin_theta,
            self.cos_theta,
            direction.x,
            direction.y,
            direction.z,
        );

        Ray::new_timed(origin, direction, ray.time())
    }

    /// This is the opposite of [[self.to_object_space]]. It uses the formula for rotating with theta.
    fn to_world_space(&self, record: &HitRecord) -> (Point, UnitVec3D) {
        let p = record.p();
        let normal = record.normal();

        let p = Self::rotate(&self.kind, self.sin_theta, self.cos_theta, p.x, p.y, p.z);

        let rotated_normal = Self::rotate(
            &self.kind,
            self.sin_theta,
            self.cos_theta,
            normal.0.x,
            normal.0.y,
            normal.0.z,
        );
        let normal = UnitVec3D(rotated_normal);

        (p, normal)
    }

    fn rotate<K>(
        kind: &RotateKind,
        sin_theta: Real,
        cos_theta: Real,
        x: Real,
        y: Real,
        z: Real,
    ) -> VecLike<K> {
        let (new_x, new_y, new_z) = match kind {
            RotateKind::X => {
                let (new_y, new_z) = Self::rotate_x(sin_theta, cos_theta, y, z);
                (x, new_y, new_z)
            }
            RotateKind::Y => {
                let (new_x, new_z) = Self::rotate_y(sin_theta, cos_theta, x, z);
                (new_x, y, new_z)
            }
            RotateKind::Z => {
                let (new_x, new_y) = Self::rotate_z(sin_theta, cos_theta, x, y);
                (new_x, new_y, z)
            }
        };
        VecLike::new(new_x, new_y, new_z)
    }

    /// Rotation around y is defined by the following formulas:
    ///     1. `x = cos(theta) * x + sin(theta) * z`
    ///     2. `z = -sin(theta) * x + cos(theta) * z`
    fn rotate_y(sin_theta: Real, cos_theta: Real, x: Real, z: Real) -> (Real, Real) {
        let new_x = cos_theta * x + sin_theta * z;
        let new_z = -sin_theta * x + cos_theta * z;
        (new_x, new_z)
    }

    /// Rotation around x is defined by the following formulas:
    ///     1. `y = cos(theta) * y - sin(theta) * z`
    ///     2. `z = sin(theta) * y + cos(theta) * z`
    fn rotate_x(sin_theta: Real, cos_theta: Real, y: Real, z: Real) -> (Real, Real) {
        let new_y = cos_theta * y - sin_theta * z;
        let new_z = sin_theta * y + cos_theta * z;
        (new_y, new_z)
    }

    /// Rotation around z is defined by the following formulas:
    ///     1. `x = cos(theta) * x - sin(theta) * y`
    ///     2. `y = sin(theta) * x + cos(theta) * y`
    fn rotate_z(sin_theta: Real, cos_theta: Real, x: Real, y: Real) -> (Real, Real) {
        let new_x = cos_theta * x - sin_theta * y;
        let new_y = sin_theta * x + cos_theta * y;
        (new_x, new_y)
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
