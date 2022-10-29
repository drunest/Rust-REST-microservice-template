use sqlx::postgres::PgPoolOptions;

pub async fn create_pool() -> sqlx::PgPool {
    let connection_string =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable missing");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&connection_string)
        .await
        .expect("Failed to create connection pool")
}
