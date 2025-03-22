use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::{
    db::AppState,
    handlers::{create_note, get_health, get_notes},
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(get_health))
        .route("/api/create", post(post(create_note)))
        .route("/api/note/:id", get(get(get_notes)))
        .with_state(app_state)
}
