use crate::define_metric;
use std::sync::atomic::{AtomicBool, Ordering};

define_metric!(AABB_HIT_COUNT);
define_metric!(BVH_HIT_COUNT);
define_metric!(OBJECT_HIT_COUNT);
define_metric!(LEFT_NODE_HIT_COUNT);
define_metric!(RIGHT_NODE_HIT_COUNT);

pub static METRICS_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn enable_metrics(flag: bool) {
    METRICS_ENABLED.store(flag, Ordering::Relaxed);
}

pub fn metrics_enabled() -> bool {
    METRICS_ENABLED.load(Ordering::Relaxed)
}

pub fn report() {
    report_aabb_hit_count();
    report_object_hit_count();
    if metrics_enabled() {
        AABB_HIT_COUNT.with(|aabb_hit_count| {
            OBJECT_HIT_COUNT.with(|object_hit_count| {
                let ratio = aabb_hit_count.get() as f64 / object_hit_count.get() as f64;

                if ratio < 4.0 {
                    log::info!("AABB/Object ratio: {:.2}", ratio);
                } else {
                    log::warn!("AABB/Object ratio too big: {ratio}")
                }
            })
        });
    }
    report_bvh_hit_count();
    report_left_node_hit_count();
    report_right_node_hit_count();
}
