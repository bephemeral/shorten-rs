use sqlx::PgPool;

pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> AppState {
        AppState { pool }
    }
}
