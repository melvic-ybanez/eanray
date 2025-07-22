use crate::core::bvh::BVH;
use crate::core::hit::ObjectRef;
use crate::core::Hittable;
use crate::define_flag;
use std::fmt::Display;

define_flag!(STATS_ENABLED);

#[derive(Default)]
pub struct BVHStats {
    internal_node_count: u32,
    leaf_count: u32,
    // number of edges from the root to the farthest leaf
    max_depth: u32,
    depth_sum: u32,
    primitive_count: u32,
}

impl BVHStats {
    pub fn from_bvh(bvh: &BVH) -> Self {
        let mut this: Self = Default::default();
        this.inspect_bvh(bvh, 0);
        this
    }

    fn inspect_bvh(&mut self, bvh: &BVH, depth: u32) {
        self.internal_node_count += 1;
        self.inspect_node(bvh.left(), depth + 1);
        self.inspect_node(bvh.right(), depth + 1);
    }

    fn inspect_node(&mut self, node: ObjectRef, depth: u32) {
        self.inspect_hittable(&*node, depth);
    }

    fn inspect_hittable(&mut self, hittable: &Hittable, depth: u32) {
        match hittable {
            Hittable::Sphere(_)
            | Hittable::Planar(_)
            | Hittable::Translate(_)
            | Hittable::RotateY(_) => {
                self.leaf_count += 1;
                if depth > self.max_depth {
                    self.max_depth = depth;
                }
                self.depth_sum += depth;
                self.primitive_count += 1;
            }
            Hittable::List(list) => list
                .objects()
                .iter()
                .for_each(|object| self.inspect_hittable(object, depth)),
            Hittable::BVH(bvh) => {
                self.inspect_bvh(bvh, depth);
            }
        }
    }

    fn average_depth(&self) -> f32 {
        self.depth_sum as f32 / self.leaf_count as f32
    }

    fn average_primitives_per_leaf(&self) -> f32 {
        self.primitive_count as f32 / self.leaf_count as f32
    }

    pub fn report(&self) {
        if !is_enabled() {
            return;
        }

        fn log_or_warn_ideal<E: Display, G: Display>(
            label: &str,
            expected: E,
            got: G,
            is_ideal: bool,
        ) {
            if is_ideal {
                log::info!("{label}: {got}");
            } else {
                log::warn!("Ideal {}: {}. Got: {}", label.to_lowercase(), expected, got);
            }
        }

        log::info!(
            "Total node count: {}",
            self.internal_node_count + self.leaf_count
        );

        log_or_warn_ideal(
            "Internal node count",
            self.leaf_count - 1,
            self.internal_node_count,
            self.internal_node_count == self.leaf_count - 1,
        );

        log::info!("Leaf node count: {}", self.leaf_count);
        log::info!("Primitive count: {}", self.primitive_count);

        let ideal_max_depth = (self.leaf_count as f32).log2().ceil() + 2.0;
        let max_depth = self.max_depth as f32;
        log_or_warn_ideal(
            "Max depth",
            ideal_max_depth,
            max_depth,
            max_depth <= ideal_max_depth,
        );

        let ideal_average_depth = 1.2 * (self.primitive_count as f32).log2();
        let average_depth = self.average_depth();
        log_or_warn_ideal(
            "Average depth",
            ideal_average_depth,
            average_depth,
            average_depth <= ideal_average_depth,
        );

        let avg_primitives_per_leaf = self.average_primitives_per_leaf();
        let ideal_primitives_per_leaf = self.primitive_count as f32
            / (self.primitive_count as f32 / BVH::PRIMITIVE_COUNT_PER_LEAF as f32).ceil();
        log_or_warn_ideal(
            "Average primitives per leaf",
            ideal_primitives_per_leaf,
            avg_primitives_per_leaf,
            avg_primitives_per_leaf / ideal_primitives_per_leaf > 0.9,
        );
    }
}

pub fn report(hittable: &Hittable) {
    if !is_enabled() {
        return;
    }

    match hittable {
        Hittable::BVH(bvh) => BVHStats::from_bvh(bvh).report(),
        Hittable::List(agg) => agg.objects().iter().for_each(report),
        _ => (),
    }
}
