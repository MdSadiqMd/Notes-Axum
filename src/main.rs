use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use db::AppState;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod db;
mod handlers;
mod model;
mod routes;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let pool = db::connect().await;
    let state = Arc::new(AppState { db: pool });
    let app = routes::create_router(state).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("🚀 Server started on 0.0.0.0:8000");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
