extern crate actix_cors;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate sqlx;

mod cors;
mod database;
mod handler;
mod models;

use actix_web::{middleware, web, App, HttpServer};

use cors::create_cors;
use database::create_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let pool = create_pool().await;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // FIXME: Path normalizer makes shit even worst
            //.wrap(middleware::NormalizePath::default())
            .wrap(create_cors())
            .data(pool.clone())
            .configure(handler::register)
            .default_service(web::to(|| async { "404" }))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
