use crate::core::math::{self, Real};
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Interval {
    pub(crate) min: Real,
    pub(crate) max: Real,
}

impl Interval {
    pub(crate) fn new(min: Real, max: Real) -> Self {
        Self { min, max }
    }

    pub(crate) fn empty() -> Self {
        Self::new(math::INFINITY, -math::INFINITY)
    }

    pub(crate) fn universe() -> Self {
        Self::new(-math::INFINITY, math::INFINITY)
    }

    pub(crate) fn from_intervals(a: &Interval, b: &Interval) -> Self {
        Self::new(a.min.min(b.min), a.max.max(b.max))
    }

    pub(crate) fn size(&self) -> Real {
        self.max - self.min
    }

    pub(crate) fn contains(&self, x: Real) -> bool {
        self.min <= x && x <= self.max
    }

    pub(crate) fn surrounds(&self, x: Real) -> bool {
        self.min < x && x < self.max
    }

    pub(crate) fn clamp(&self, x: Real) -> Real {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub(crate) fn expand(&self, delta: Real) -> Interval {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }
}

impl Add<Real> for &Interval {
    type Output = Interval;

    fn add(self, displacement: Real) -> Self::Output {
        Interval::new(self.min + displacement, self.max + displacement)
    }
}

impl Add<&Interval> for Real {
    type Output = Interval;

    fn add(self, rhs: &Interval) -> Self::Output {
        rhs + self
    }
}
