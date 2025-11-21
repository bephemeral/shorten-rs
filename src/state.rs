use crate::models::link::Link;
use dashmap::DashMap;
use std::sync::atomic::AtomicUsize;

pub struct AppState {
    pub redirects: DashMap<usize, Link>,
    pub last_id: AtomicUsize,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            redirects: DashMap::new(),
            last_id: AtomicUsize::new(0),
        }
    }
}
