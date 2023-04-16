use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: DatabaseConnection,
}
