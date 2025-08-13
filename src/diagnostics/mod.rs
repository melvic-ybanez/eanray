use crate::diagnostics::macros::define_flag;
use crate::settings::DiagnosticsConfig;

pub mod metrics;

pub mod macros;
pub mod stats;

define_flag!(DIAGNOSTICS_ENABLED);

pub fn enable_all(flag: bool) {
    metrics::enable(flag);
    stats::enable(flag);
}

pub fn setup(diagnostics: &DiagnosticsConfig) {
    metrics::enable(diagnostics.enable_metrics());
    stats::enable(diagnostics.enable_stats());
}
