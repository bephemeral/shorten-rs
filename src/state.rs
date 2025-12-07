use crate::link::Link;
use dashmap::DashMap;
use sqlx::PgPool;

pub struct AppState {
    pub pool: PgPool,
    pub redirects: DashMap<String, Link>,
}

impl AppState {
    pub fn new(pool: PgPool) -> AppState {
        AppState {
            pool,
            redirects: DashMap::new(),
        }
    }
}
