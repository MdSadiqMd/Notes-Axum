use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::{
    db::AppState,
    handlers::{create_note, get_health},
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(get_health))
        .route("/api/create", post(post(create_note)))
        .with_state(app_state)
}
