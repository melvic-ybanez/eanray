#[macro_export]
macro_rules! define_metric {
    ($name: ident) => {
        thread_local! {
            static $name: std::cell::Cell<u64> = std::cell::Cell::new(0);
        }

        paste::item! {
            #[inline]
            pub fn [<increment_$name:lower>]() {
                if $crate::diagnostics::metrics::metrics_enabled() {
                    $name.with(|c| c.set(c.get() + 1));
                }
            }

            #[inline]
            pub fn [<get_$name>]() -> Option<u64> {
                if $crate::diagnostics::metrics::metrics_enabled() {
                    Some($name.with(|c| c.get()))
                } else {
                    None
                }
            }
            
            #[inline]
            pub fn [<report_$name:lower>]() {
                if $crate::diagnostics::metrics::metrics_enabled() {
                    let label = stringify!($name);
                    let label = label.replace("_", " ");
                    $name.with(|c| println!("{label}: {}", c.get()))
                }
            }
        }
    };
}