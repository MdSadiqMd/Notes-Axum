use axum::{routing::get, Router};
use std::sync::Arc;

use crate::{db::AppState, handlers::get_health};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(get_health))
        .with_state(app_state)
}
