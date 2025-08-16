use crate::diagnostics::macros::define_flag;
use crate::settings::DiagnosticsConfig;

pub(crate) mod metrics;

pub(crate) mod macros;
pub(crate) mod stats;

define_flag!(DIAGNOSTICS_ENABLED);

pub(crate) fn enable_all(flag: bool) {
    metrics::enable(flag);
    stats::enable(flag);
}

pub(crate) fn setup(diagnostics: &DiagnosticsConfig) {
    metrics::enable(diagnostics.enable_metrics());
    stats::enable(diagnostics.enable_stats());
}
