use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::{
    db::AppState,
    model::NoteModel,
    schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
};

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

pub async fn get_note(
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

pub async fn get_all_notes(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as("SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2")
        .bind(limit as i32)
        .bind(offset as i32)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "500",
            "message": "Something bad happened while fetching all note items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let notes: NoteModel = query_result.unwrap();
    let json_response = serde_json::json!({
        "status": "200",
        "results": notes.title.len(),
        "notes": notes
    });
    Ok(Json(json_response))
}

pub async fn update_note(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result: Result<NoteModel, sqlx::Error> =
        sqlx::query_as::<_, NoteModel>("SELECT * FROM notes WHERE id = $1")
            .bind(id)
            .fetch_one(&data.db)
            .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "500",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    };

    let now = chrono::Utc::now();
    let note = query_result.unwrap();
    let query_result: Result<NoteModel, sqlx::Error> = sqlx::query_as::<_, NoteModel>(
        "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *"
    )
        .bind(body.title.to_owned().unwrap_or(note.title))
        .bind(body.content.to_owned().unwrap_or(note.content))
        .bind(body.category.to_owned().unwrap_or(note.category.unwrap()))
        .bind(body.published.unwrap_or(note.published.unwrap().to_string()))
        .bind(now)
        .bind(id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(note) => {
            return Ok(Json({
                serde_json::json!({
                    "status":"200",
                    "message":serde_json::json!({
                        "note":note
                    })
                })
            }));
        }
        Err(err) => {
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

pub async fn delete_note(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result: Result<NoteModel, sqlx::Error> =
        sqlx::query_as("DELETE FROM notes WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(&data.db)
            .await;

    match query_result {
        Ok(note) => {
            return Ok(Json({
                serde_json::json!({
                    "status":"200",
                    "message":serde_json::json!({
                        "note":note
                    })
                })
            }));
        }
        Err(err) => {
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
