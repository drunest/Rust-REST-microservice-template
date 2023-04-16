use crate::state::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use entity::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TryIntoModel};
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct CreatePayload {
    name: String,
}

#[derive(serde::Deserialize)]
pub struct UpdatePayload {
    name: String,
}

pub async fn create_one(
    state: State<Arc<AppState>>,
    Json(create_payload): Json<CreatePayload>,
) -> Result<Json<entity::example::Model>, StatusCode> {
    entity::example::ActiveModel {
        name: Set(create_payload.name),
        ..Default::default()
    }
    .insert(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    .map(Json)
}

pub async fn get_one(
    state: State<Arc<AppState>>,
    Path(example_id): Path<i32>,
) -> Result<Json<entity::example::Model>, StatusCode> {
    if let Some(entry) = Example::find_by_id(example_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        Ok(Json(entry))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all(
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<entity::example::Model>>, StatusCode> {
    Example::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(Json)
}

pub async fn delete_one(
    state: State<Arc<AppState>>,
    Path(example_id): Path<i32>,
) -> impl IntoResponse {
    Example::delete_by_id(example_id)
        .exec(&state.db)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_one(
    state: State<Arc<AppState>>,
    Path(example_id): Path<i32>,
    Json(update_payload): Json<UpdatePayload>,
) -> Result<Json<entity::example::Model>, StatusCode> {
    if let Some(existing_entry) = Example::find_by_id(example_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let mut existing_entry: entity::example::ActiveModel = existing_entry.into();
        existing_entry.name = Set(update_payload.name);
        if existing_entry.is_changed() {
            existing_entry
                .update(&state.db)
                .await
                .map(Json)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        } else {
            existing_entry
                .try_into_model()
                .map(Json)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
