use std::cell::Cell;
use crate::diagnostics::logger;

thread_local! {
    pub static AABB_HIT_COUNT: Cell<usize> = Cell::new(0);
    pub static OBJECT_HIT_COUNT: Cell<usize> = Cell::new(0);
    pub static LEFT_NODE_HIT_COUNT: Cell<usize> = Cell::new(0);
    pub static RIGHT_NODE_HIT_COUNT: Cell<usize> = Cell::new(0);
}

pub fn report() {
    fn report_count(label: &str) -> impl Fn(&Cell<usize>) {
        move |c| println!("{label} hit count: {}", c.get())
    }

    AABB_HIT_COUNT.with(report_count("AABB"));
    OBJECT_HIT_COUNT.with(report_count("Object"));
    AABB_HIT_COUNT.with(|aabb_hit_count| {
        OBJECT_HIT_COUNT.with(|object_hit_count| {
            let ratio = aabb_hit_count.get() as f64 / object_hit_count.get() as f64;

            if ratio < 4.0 {
                logger::info(format!("AABB/Object ratio: {:.2}", ratio).as_str());
            } else {
                logger::warning(format!("AABB/Object ratio too big: {ratio}").as_str())
            }
        })
    });
    LEFT_NODE_HIT_COUNT.with(report_count("Left nodes"));
    RIGHT_NODE_HIT_COUNT.with(report_count("Right nodes"));
}

pub fn bump_count() -> impl Fn(&Cell<usize>) {
    |c| c.set(c.get() + 1)
}
