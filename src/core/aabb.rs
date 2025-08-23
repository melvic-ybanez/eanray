use crate::core::Ray;
use crate::core::math::interval::Interval;
use crate::core::math::{Axis, Point, Vec3D};
use crate::diagnostics::metrics;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::Add;

/// Axis-aligned Bounding Box
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub(crate) fn empty() -> Self {
        Self::new(Interval::empty(), Interval::empty(), Interval::empty())
    }

    pub(crate) fn universe() -> Self {
        Self::new(
            Interval::universe(),
            Interval::universe(),
            Interval::universe(),
        )
    }

    pub(crate) fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut this = Self { x, y, z };
        this.pad_to_minimums();
        this
    }

    pub(crate) fn from_points(a: Point, b: Point) -> Self {
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

    pub(crate) fn from_boxes(box0: &AABB, box1: &AABB) -> Self {
        Self::new(
            Interval::from_intervals(&box0.x, &box1.x),
            Interval::from_intervals(&box0.y, &box1.y),
            Interval::from_intervals(&box0.z, &box1.z),
        )
    }

    pub(crate) fn axis_interval(&self, axis: &Axis) -> &Interval {
        match axis {
            Axis::Y => &self.y,
            Axis::X => &self.x,
            Axis::Z => &self.z,
        }
    }

    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        metrics::increment_aabb_hit_attempt_count();

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
                if t0 > t_min {
                    t_min = t0
                }
                if t1 < t_max {
                    t_max = t1
                }
            } else {
                if t1 > t_min {
                    t_min = t1
                }
                if t0 < t_max {
                    t_max = t0
                }
            }

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub(crate) fn longest_axis(&self) -> Axis {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                Axis::X
            } else {
                Axis::Z
            }
        } else if self.y.size() > self.z.size() {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }

    pub(crate) fn x(&self) -> &Interval {
        &self.x
    }

    pub(crate) fn y(&self) -> &Interval {
        &self.y
    }

    pub(crate) fn z(&self) -> &Interval {
        &self.z
    }
}

impl Add<&Vec3D> for &AABB {
    type Output = AABB;

    fn add(self, offset: &Vec3D) -> Self::Output {
        AABB::new(&self.x + offset.x, &self.y + offset.y, &self.z + offset.z)
    }
}

impl Add<&AABB> for &Vec3D {
    type Output = AABB;

    fn add(self, bbox: &AABB) -> Self::Output {
        bbox + self
    }
}
