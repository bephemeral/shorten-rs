pub mod models;
pub mod routes;

use std::sync::atomic::AtomicUsize;

use dashmap::DashMap;

use crate::models::link::Link;

pub struct AppState {
    redirects: DashMap<usize, Link>,
    last_id: AtomicUsize,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            redirects: DashMap::new(),
            last_id: AtomicUsize::new(0),
        }
    }
}
