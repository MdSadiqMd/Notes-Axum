use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;

use crate::{
    db::AppState,
    handlers::{create_note, delete_note, get_all_notes, get_health, get_note, update_note},
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(get_health))
        .route("/api/create", post(post(create_note)))
        .route("/api/note/:id", get(get(get_note)))
        .route("/api/notes", get(get(get_all_notes)))
        .route("/api/notes/:id", patch(patch(update_note)))
        .route("/api/notes/:id", delete(delete(delete_note)))
        .with_state(app_state)
}
