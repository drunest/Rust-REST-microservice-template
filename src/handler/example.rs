use crate::models::example::{Example, InsertExample, UpdateExample};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;

#[post("/")]
pub async fn create_one(
    pool: web::Data<PgPool>,
    create_payload: web::Json<InsertExample>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Example,
        "INSERT INTO examples (name) VALUES ($1) RETURNING *",
        create_payload.name
    )
    .fetch_one(pool.as_ref())
    .await;
    match result {
        Ok(row) => HttpResponse::Ok().json(row),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{example_id}/")]
pub async fn get_one(pool: web::Data<PgPool>, example_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query_as!(
        Example,
        "SELECT * FROM examples WHERE id = $1",
        example_id.as_ref()
    )
    .fetch_one(pool.as_ref())
    .await;
    match result {
        Ok(row) => HttpResponse::Ok().json(row),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/")]
pub async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(Example, "SELECT * FROM examples")
        .fetch_all(pool.as_ref())
        .await;
    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{example_id}/")]
pub async fn delete_one(pool: web::Data<PgPool>, example_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM examples where id = $1", example_id.as_ref())
        .execute(pool.as_ref())
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{example_id}/")]
pub async fn update_one(
    pool: web::Data<PgPool>,
    example_id: web::Path<i32>,
    update_payload: web::Json<UpdateExample>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Example,
        "UPDATE examples SET name = $1 WHERE id = $2 RETURNING *",
        update_payload.name,
        example_id.as_ref()
    )
    .fetch_one(pool.as_ref())
    .await;
    match result {
        Ok(row) => HttpResponse::Ok().json(row),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
