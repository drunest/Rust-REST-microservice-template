use crate::{
    models::example::{Example, InsertExample, UpdateExample},
    state::AppState,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

pub async fn create_one(
    state: State<Arc<AppState>>,
    Json(create_payload): Json<InsertExample>,
) -> Result<Json<Example>, StatusCode> {
    sqlx::query_as!(
        Example,
        "INSERT INTO examples (name) VALUES ($1) RETURNING *",
        create_payload.name
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    .map(Json)
}

pub async fn get_one(
    state: State<Arc<AppState>>,
    Path(example_id): Path<i32>,
) -> Result<Json<Example>, StatusCode> {
    sqlx::query_as!(Example, "SELECT * FROM examples WHERE id = $1", example_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)
        .map(Json)
}

pub async fn get_all(state: State<Arc<AppState>>) -> Result<Json<Vec<Example>>, StatusCode> {
    sqlx::query_as!(Example, "SELECT * FROM examples")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(Json)
}

pub async fn delete_one(
    state: State<Arc<AppState>>,
    Path(example_id): Path<i32>,
) -> impl IntoResponse {
    sqlx::query!("DELETE FROM examples where id = $1", example_id)
        .execute(&state.db)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_one(
    state: State<Arc<AppState>>,
    Path(example_id): Path<i32>,
    Json(update_payload): Json<UpdateExample>,
) -> Result<Json<Example>, StatusCode> {
    sqlx::query_as!(
        Example,
        "UPDATE examples SET name = $1 WHERE id = $2 RETURNING *",
        update_payload.name,
        example_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    .map(Json)
}
