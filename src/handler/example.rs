use crate::models::example::{Example, InsertExample, UpdateExample};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::{PgPool, Row};

#[post("/")]
pub async fn create_one(
    pool: web::Data<PgPool>,
    create_payload: web::Json<InsertExample>,
) -> impl Responder {
    let result = sqlx::query("INSERT INTO examples (name) VALUES ($1) RETURNING *")
        .bind(&create_payload.name)
        .fetch_one(pool.as_ref())
        .await;
    match result {
        Ok(row) => HttpResponse::Ok().json(Example {
            id: row.get(0),
            name: row.get(1),
            updated_at: row.get(2),
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{example_id}")]
pub async fn get_one(pool: web::Data<PgPool>, example_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query("SELECT * FROM examples WHERE id = $1")
        .bind(example_id.as_ref())
        .fetch_one(pool.as_ref())
        .await;
    match result {
        Ok(row) => HttpResponse::Ok().json(Example {
            id: row.get(0),
            name: row.get(1),
            updated_at: row.get(2),
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/")]
pub async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query("SELECT * FROM examples")
        .fetch_all(pool.as_ref())
        .await;
    match result {
        Ok(rows) => {
            let mut examples: Vec<Example> = vec![];
            for row in rows {
                examples.push(Example {
                    id: row.get(0),
                    name: row.get(1),
                    updated_at: row.get(2),
                })
            }
            HttpResponse::Ok().json(examples)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{example_id}")]
pub async fn delete_one(pool: web::Data<PgPool>, example_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query("DELETE FROM examples where id = $1")
        .bind(example_id.as_ref())
        .execute(pool.as_ref())
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{example_id}")]
pub async fn update_one(
    pool: web::Data<PgPool>,
    example_id: web::Path<i32>,
    update_payload: web::Json<UpdateExample>,
) -> impl Responder {
    let result = sqlx::query("UPDATE examples SET name = $1 WHERE id = $2 RETURNING *")
        .bind(&update_payload.name)
        .bind(example_id.as_ref())
        .fetch_one(pool.as_ref())
        .await;
    match result {
        Ok(row) => HttpResponse::Ok().json(Example {
            id: row.get(0),
            name: row.get(1),
            updated_at: row.get(2),
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
