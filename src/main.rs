extern crate serde;
extern crate serde_json;
extern crate sqlx;

mod database;
mod handler;
mod models;
mod state;

use axum::{routing, Router};
use database::create_pool;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, request_id::PropagateRequestIdLayer,
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "rust_rest_microservice_template=trace");
    tracing_subscriber::fmt::init();

    let pool = create_pool().await;
    let state = std::sync::Arc::new(state::AppState { db: pool.clone() });
    let app = Router::with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(CorsLayer::new())
                .layer(PropagateRequestIdLayer::x_request_id()),
        )
        .route(
            "/",
            routing::post(handler::example::create_one).get(handler::example::get_all),
        )
        .route(
            "/:example_id",
            routing::get(handler::example::get_one)
                .delete(handler::example::delete_one)
                .put(handler::example::update_one),
        );

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
