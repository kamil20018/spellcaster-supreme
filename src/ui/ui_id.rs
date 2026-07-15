use std::sync::atomic::{AtomicU64, Ordering};

static UI_ID: AtomicU64 = AtomicU64::new(0);

pub fn new_id() -> u64 {
    UI_ID.fetch_add(1, Ordering::SeqCst)
}
