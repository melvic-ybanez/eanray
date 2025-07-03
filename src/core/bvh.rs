use crate::core::aabb::AABB;
use crate::core::hit::HitRecord;
use crate::core::math::interval::Interval;
use crate::core::math::Axis;
use crate::core::{math, Hittable, HittableList, Ray};
use crate::diagnostics::metrics;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BVH<'a> {
    left: Rc<Hittable<'a>>,
    right: Rc<Hittable<'a>>,
    bbox: AABB,
}

impl<'a> BVH<'a> {
    pub fn from_list(mut list: HittableList<'a>) -> Self {
        let mut objects = list
            .objects_mut()
            .iter()
            .map(|object| Rc::new(object.clone()))
            .collect::<Vec<Rc<Hittable<'a>>>>();
        let objects = &mut objects;
        Self::from_objects(objects, 0, objects.len())
    }

    pub fn from_objects(objects: &mut Vec<Rc<Hittable<'a>>>, start: usize, end: usize) -> Self {
        metrics::increment_bvh_init_count();

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
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            (objects[start].clone(), objects[start + 1].clone())
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            let left = BVH::from_objects(objects, start, mid);
            let right = BVH::from_objects(objects, mid, end);
            (Rc::new(Hittable::BVH(left)), Rc::new(Hittable::BVH(right)))
        };

        let left_bbox = left.bounding_box();
        let right_bbox = right.bounding_box();

        Self {
            left: left.clone(),
            right: right.clone(),
            bbox: AABB::from_boxes(left_bbox, right_bbox),
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        metrics::increment_bvh_hit_attempt_count();

        if !self.bbox.hit(ray, ray_t) {
            metrics::increment_bvh_miss_count();
            return None;
        }

        let hit_right = |t_max| {
            self.right
                .hit(ray, &Interval::new(ray_t.min, t_max))
                .and_then(|hit| {
                    metrics::increment_right_node_hit_attempt_count();
                    Some(hit)
                })
        };

        if let Some(left_rec) = self.left.hit(ray, ray_t) {
            metrics::increment_left_node_hit_attempt_count();
            hit_right(left_rec.t()).or(Some(left_rec))
        } else {
            hit_right(ray_t.max)
        }
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    pub fn box_x_compare(a: &Rc<Hittable>, b: &Rc<Hittable>) -> Ordering {
        Self::box_compare(a, b, &Axis::X)
    }

    pub fn box_y_compare(a: &Rc<Hittable>, b: &Rc<Hittable>) -> Ordering {
        Self::box_compare(a, b, &Axis::Y)
    }

    pub fn box_z_compare(a: &Rc<Hittable>, b: &Rc<Hittable>) -> Ordering {
        Self::box_compare(a, b, &Axis::Z)
    }

    pub fn box_compare(a: &Rc<Hittable>, b: &Rc<Hittable>, axis_index: &Axis) -> Ordering {
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
