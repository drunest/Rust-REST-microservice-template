mod database;
mod handler;
mod state;

use axum::{routing, Router};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, request_id::PropagateRequestIdLayer,
    trace::TraceLayer,
};

#[tokio::main]
pub async fn start() {
    tracing_subscriber::fmt::init();

    let pool = database::init().await;
    database::migrate(&pool).await;

    let state = std::sync::Arc::new(state::AppState { db: pool.clone() });
    let app = Router::new()
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
        )
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
