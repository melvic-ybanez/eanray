use crate::define_metric;
use std::sync::atomic::{AtomicBool, Ordering};

define_metric!(AABB_HIT_ATTEMPT_COUNT);
define_metric!(BVH_INIT_COUNT);
define_metric!(BVH_HIT_ATTEMPT_COUNT);
define_metric!(BVH_MISS_COUNT);
define_metric!(OBJECT_HIT_ATTEMPT_COUNT);
define_metric!(LEFT_NODE_HIT_ATTEMPT_COUNT);
define_metric!(RIGHT_NODE_HIT_ATTEMPT_COUNT);

pub static METRICS_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn enable_metrics(flag: bool) {
    METRICS_ENABLED.store(flag, Ordering::Relaxed);
}

pub fn metrics_enabled() -> bool {
    METRICS_ENABLED.load(Ordering::Relaxed)
}

pub fn report() {
    report_aabb_hit_attempt_count();
    report_object_hit_attempt_count();
    if metrics_enabled() {
        AABB_HIT_ATTEMPT_COUNT.with(|aabb_hit_count| {
            OBJECT_HIT_ATTEMPT_COUNT.with(|object_hit_count| {
                let ratio = aabb_hit_count.get() as f64 / object_hit_count.get() as f64;
                let message_prefix = "AABB/Object hit attempts ratio";

                if ratio > 3.0 {
                    log::warn!("{message_prefix} is too high: {ratio}");
                } else if ratio < 1.5 {
                    log::warn!("{message_prefix} is too low: {ratio}")
                } else {
                    log::info!("{message_prefix}: {ratio}")
                }
            })
        });
    }
    report_bvh_init_count();
    report_bvh_hit_attempt_count();
    report_bvh_miss_count();
    report_left_node_hit_attempt_count();
    report_right_node_hit_attempt_count();
}
