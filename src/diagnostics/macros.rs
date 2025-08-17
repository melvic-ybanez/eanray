macro_rules! define_flag {
    ($name: ident) => {
        pub(crate) static $name: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);

        pub(crate) fn enable(flag: bool) {
            $name.store(flag, std::sync::atomic::Ordering::Relaxed);
        }

        pub(crate) fn is_enabled() -> bool {
            $name.load(std::sync::atomic::Ordering::Relaxed)
        }
    };
}

macro_rules! define_metric {
    ($name: ident) => {
        thread_local! {
            static $name: std::cell::Cell<u64> = std::cell::Cell::new(0);
        }

        paste::item! {
            #[inline]
            pub(crate) fn [<increment_$name:lower>]() {
                if $crate::diagnostics::metrics::is_enabled() {
                    $name.with(|c| c.set(c.get() + 1));
                }
            }

            #[inline]
            pub(crate) fn [<get_$name:lower>]() -> Option<u64> {
                if $crate::diagnostics::metrics::is_enabled() {
                    Some($name.with(|c| c.get()))
                } else {
                    None
                }
            }

            #[inline]
            pub(crate) fn [<report_$name:lower>]() {
                if $crate::diagnostics::metrics::is_enabled() {
                    let label = stringify!($name);
                    let label = label.replace("_", " ");
                    $name.with(|c| log::info!("{label}: {}", c.get()))
                }
            }
        }
    };
}

pub(crate) use define_flag;
pub(crate) use define_metric;
