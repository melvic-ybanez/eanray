use crate::core::aabb::AABB;
use crate::core::hittables::ObjectRef;
use crate::core::math::matrix::{matrix_4x4, Matrix};
use crate::core::math::{Axis, Point, Real};
use crate::core::{math, Hittable};

#[derive(Clone, Debug)]
pub(crate) struct Transform {
    forward: Matrix,
    inverse: Matrix,
    normal: Matrix,
    pub(crate) object: ObjectRef,
    bbox: AABB,
}

impl Transform {
    pub(crate) fn new(object: ObjectRef, forward: Matrix) -> Self {
        let identity = matrix_4x4::identity();

        let (forward, object) = match &*object {
            Hittable::Transform(transform) => (
                forward * transform.forward.clone(),
                transform.object.clone(),
            ),
            _ => (forward, object),
        };

        let bbox = object.bounding_box().clone();

        let mut this = Self {
            object,
            forward,
            inverse: identity.clone(),
            normal: identity.clone(),
            bbox,
        };

        this.recompute();
        this.compute_bounding_box();
        this
    }

    pub(crate) fn recompute(&mut self) {
        self.inverse = self.forward.inverse().unwrap();
        self.normal = self.inverse.transpose();
    }

    pub(crate) fn forward(&self) -> &Matrix {
        &self.forward
    }

    pub(crate) fn inverse(&self) -> &Matrix {
        &self.inverse
    }

    pub(crate) fn normal(&self) -> &Matrix {
        &self.normal
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    pub(crate) fn compute_bounding_box(&mut self) {
        let mut min = Point::from_scalar(math::INFINITY);
        let mut max = Point::from_scalar(-math::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = math::lerp(self.bbox.x().min, self.bbox.x().max, i as Real);
                    let y = math::lerp(self.bbox.y().min, self.bbox.y().max, j as Real);
                    let z = math::lerp(self.bbox.z().min, self.bbox.z().max, k as Real);

                    let world_space = Point::new(x, y, z).transform(&self.forward);

                    for c in 0..3 {
                        let c = Axis::from_usize_unsafe(c);
                        min[&c] = min[&c].min(world_space[&c]);
                        max[&c] = max[&c].max(world_space[&c]);
                    }
                }
            }
        }

        self.bbox = AABB::from_points(min, max);
    }
}
