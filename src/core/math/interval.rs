use crate::core::math::{self, Real};

pub struct Interval {
    min: Real,
    max: Real,
}

impl Interval {
    pub fn new(min: Real, max: Real) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self::new(math::INFINITY, -math::INFINITY)
    }

    pub fn universe() -> Self {
        Self::new(-math::INFINITY, math::INFINITY)
    }

    pub fn size(&self) -> Real {
        self.max - self.min
    }

    pub fn contains(&self, x: Real) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: Real) -> bool {
        self.min < x && x < self.max
    }

    pub fn min(&self) -> Real {
        self.min
    }

    pub fn max(&self) -> Real {
        self.max
    }
    
    pub fn clamp(&self, x: Real) -> Real {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
