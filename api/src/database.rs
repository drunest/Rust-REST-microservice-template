use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

pub async fn init() -> DatabaseConnection {
    let connection_string =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable missing");
    Database::connect(&connection_string)
        .await
        .expect("Failed to connect to database")
}

pub async fn migrate(db: &DatabaseConnection) {
    Migrator::up(db, None).await.expect("Migration failed");
}
