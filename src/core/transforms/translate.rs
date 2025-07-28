use serde::{Deserialize, Serialize};
use crate::core::aabb::AABB;
use crate::core::hit::{HitRecord, ObjectRef};
use crate::core::math::interval::Interval;
use crate::core::math::Vec3D;
use crate::core::Ray;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Translate {
    object: ObjectRef,
    offset: Vec3D,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: ObjectRef, offset: Vec3D) -> Self {
        let bbox = object.bounding_box() + &offset;
        Self {
            object,
            offset,
            bbox,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let offset_origin = ray.origin() - &self.offset;
        let offset_ray = Ray::new_timed(offset_origin, ray.direction().clone(), ray.time());
        let mut hit_record = self.object.hit(&offset_ray, ray_t)?;
        hit_record.p = hit_record.p + &self.offset;
        Some(hit_record)
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}