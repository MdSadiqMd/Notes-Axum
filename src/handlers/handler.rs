use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::{db::AppState, model::NoteModel, schema::CreateNoteSchema};

pub async fn get_health() -> impl IntoResponse {
    return Json(serde_json::json!({
        "status":"200",
        "message":"Up and Running really fast"
    }));
}

pub async fn create_note(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result: Result<NoteModel, sqlx::Error> = sqlx::query_as(
        "INSERT INTO notes (title, content, category) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(body.title.to_string())
    .bind(body.content.to_string())
    .bind(body.category.to_owned().unwrap_or("".to_string()))
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({
                "status":"200",
                "data":json!({
                    "note":note
                })
            });
            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(err) => {
            if err
                .to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "409",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "500",
                    "message": format!("{:?}", err)
                })),
            ));
        }
    }
}

pub async fn get_notes(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result: Result<NoteModel, sqlx::Error> =
        sqlx::query_as("SELECT * FROM notes WHERE id=$1")
            .bind(id)
            .fetch_one(&data.db)
            .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({
                    "status": "200",
                    "data": serde_json::json!({
                    "note": note
                })
            });
            return Ok(Json(note_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "500",
                "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}
