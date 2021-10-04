use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Example {
    pub id: i32,
    pub name: String,
    pub updated_at: NaiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct InsertExample {
    pub name: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct UpdateExample {
    pub name: String,
}
