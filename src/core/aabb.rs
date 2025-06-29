use crate::core::math::interval::Interval;
use crate::core::math::{Axis, Point};
use crate::core::Ray;
use serde::{Deserialize, Serialize};

/// Axis-aligned Bounding Box
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn empty() -> Self {
        Self::new(Interval::empty(), Interval::empty(), Interval::empty())
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point, b: Point) -> Self {
        Self::new(
            if a.x <= b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },
            if a.y <= b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },
            if a.z <= b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        )
    }

    pub fn from_boxes(box0: &AABB, box1: &AABB) -> Self {
        Self::new(
            Interval::from_intervals(&box0.x, &box1.x),
            Interval::from_intervals(&box0.y, &box1.y),
            Interval::from_intervals(&box0.z, &box1.z),
        )
    }

    pub fn axis_interval(&self, axis: &Axis) -> &Interval {
        match axis {
            Axis::Y => &self.y,
            Axis::X => &self.x,
            Axis::Z => &self.z,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let ray_orig = ray.origin();
        let ray_dir = ray.direction();

        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;

        for axis in 0..3 {
            let axis = &Axis::from_usize_unsafe(axis);
            let ax = self.axis_interval(axis);
            let dir_inverse = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * dir_inverse;
            let t1 = (ax.max - ray_orig[axis]) * dir_inverse;

            if t0 < t1 {
                t_min = t0.max(ray_t.min);
                t_max = t1.min(ray_t.max);
            } else {
                t_min = t1.max(ray_t.min);
                t_max = t0.min(ray_t.max);
            }

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
