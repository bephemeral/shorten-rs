pub mod routes;

use std::sync::atomic::AtomicUsize;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
struct Link {
    url: String,
}
