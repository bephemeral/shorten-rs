use crate::link::Link;
use dashmap::DashMap;

pub struct AppState {
    pub redirects: DashMap<String, Link>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            redirects: DashMap::new(),
        }
    }
}
