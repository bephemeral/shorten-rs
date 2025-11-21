use crate::link::Link;
use dashmap::DashMap;
use uuid::Uuid;

pub struct AppState {
    pub redirects: DashMap<Uuid, Link>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            redirects: DashMap::new(),
        }
    }
}
