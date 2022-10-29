use sqlx::{Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
}
