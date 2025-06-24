use crate::core::aabb::AABB;
use crate::core::hit::HitRecord;
use crate::core::math::interval::Interval;
use crate::core::{math, Hittable, HittableList, Ray};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use crate::core::math::Axis;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BVH<'a> {
    left: Box<Hittable<'a>>,
    right: Box<Hittable<'a>>,
    bbox: AABB,
}

impl<'a> BVH<'a> {
    pub fn from_list(mut list: HittableList<'a>) -> Self {
        let objects = list.objects_mut();
        Self::from_objects(objects, 0, objects.len())
    }

    pub fn from_objects(objects: &mut Vec<Hittable<'a>>, start: usize, end: usize) -> Self {
        let axis = math::random_int(0, 2);

        let comparator = if axis == Axis::X as i32 {
            Self::box_x_compare
        } else if axis == Axis::Y as i32 {
            Self::box_y_compare
        } else {
            Self::box_z_compare
        };

        let object_span = end - start;
        let (left, right) = if object_span == 1 {
            (&objects[start], &objects[start])
        } else if object_span == 2 {
            (&objects[start], &objects[start + 1])
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            let left = BVH::from_objects(objects, start, mid);
            let right = BVH::from_objects(objects, mid, end);
            (&Hittable::BVH(left), &Hittable::BVH(right))
        };

        Self {
            left: Box::new(left.clone()),
            right: Box::new(right.clone()),
            bbox: AABB::from_boxes(left.bounding_box(), right.bounding_box()),
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, ray_t) {
            return None;
        }

        self.left
            .hit(ray, ray_t)
            .and_then(|left_rec| {
                // see if there's an even closer hit
                self.right
                    .hit(ray, &Interval::new(ray_t.min, left_rec.t()))
                    .or(Some(left_rec))
            })
            .or(self.right.hit(ray, &Interval::new(ray_t.min, ray_t.max)))
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    pub fn box_x_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, &Axis::X)
    }

    pub fn box_y_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, &Axis::Y)
    }

    pub fn box_z_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, &Axis::Z)
    }

    pub fn box_compare(a: &Hittable, b: &Hittable, axis_index: &Axis) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        if a_axis_interval.min < b_axis_interval.min {
            Ordering::Less
        } else if b_axis_interval.min < a_axis_interval.min {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
