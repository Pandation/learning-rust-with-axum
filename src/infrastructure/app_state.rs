use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: Pool<Postgres>,
}

impl AppState {
    pub fn new(database: Pool<Postgres>) -> Self {
        Self { database }
    }
}