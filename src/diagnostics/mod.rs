use crate::define_flag;

pub mod metrics;

pub mod macros;
pub mod stats;

define_flag!(DIAGNOSTICS_ENABLED);

pub fn enable_all(flag: bool) {
    metrics::enable(flag);
    stats::enable(flag);
}