use crate::core::aabb::AABB;
use crate::core::hittables::{HitRecord, ObjectRef};
use crate::core::math::interval::Interval;
use crate::core::math::Axis;
use crate::core::{Hittable, HittableList, Ray};
use crate::diagnostics::metrics;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BVH {
    left: ObjectRef,
    right: ObjectRef,
    bbox: AABB,
}

impl BVH {
    pub const PRIMITIVE_COUNT_PER_LEAF: u32 = 1;

    /// Unevaluated BVH. The hittable is stored in one of the children.
    /// The bounding box is empty.
    /// Maybe we could have improved this by defining an actual type for Lazy BVHs?
    pub fn lazy(hittable: Hittable) -> Self {
        let mut this = Self {
            left: Arc::new(Hittable::List(HittableList::empty())),
            right: Arc::new(Hittable::List(HittableList::empty())),
            bbox: AABB::empty(),
        };
        this.store_hittable(hittable);
        this
    }

    pub(super) fn from_list(mut list: HittableList) -> Self {
        let mut objects = list
            .objects_mut()
            .iter()
            .map(|object| {
                if !object.is_finite() {
                    log::warn!("Infinite bounding box found.")
                }

                Arc::new(object.clone())
            })
            .collect::<Vec<ObjectRef>>();
        let objects = &mut objects;
        Self::from_objects(objects, 0, objects.len())
    }

    pub(super) fn from_objects(objects: &mut Vec<ObjectRef>, start: usize, end: usize) -> Self {
        metrics::increment_bvh_init_count();

        let mut bbox = AABB::empty();
        for i in start..end {
            bbox = AABB::from_boxes(&bbox, objects[i].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = if axis == Axis::X {
            Self::box_x_compare
        } else if axis == Axis::Y {
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
            (
                Arc::new(Hittable::BVH(left)),
                Arc::new(Hittable::BVH(right)),
            )
        };

        Self {
            left: left.clone(),
            right: right.clone(),
            bbox,
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

    pub fn box_x_compare(a: &ObjectRef, b: &ObjectRef) -> Ordering {
        Self::box_compare(a, b, &Axis::X)
    }

    pub fn box_y_compare(a: &ObjectRef, b: &ObjectRef) -> Ordering {
        Self::box_compare(a, b, &Axis::Y)
    }

    pub fn box_z_compare(a: &ObjectRef, b: &ObjectRef) -> Ordering {
        Self::box_compare(a, b, &Axis::Z)
    }

    pub fn box_compare(a: &ObjectRef, b: &ObjectRef, axis_index: &Axis) -> Ordering {
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

    pub fn left(&self) -> ObjectRef {
        self.left.clone()
    }

    pub fn right(&self) -> ObjectRef {
        self.right.clone()
    }

    fn store_hittable(&mut self, hittable: Hittable) {
        self.left = Arc::new(hittable);
    }

    pub(super) fn as_hittable(&self) -> &Hittable {
        &self.left
    }
}