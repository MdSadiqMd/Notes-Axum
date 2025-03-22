use axum::{response::IntoResponse, Json};

pub async fn get_health() -> impl IntoResponse {
    return Json(serde_json::json!({
        "status":"200",
        "message":"Up and Running really fast"
    }));
}

pub async fn create_note() -> impl IntoResponse{
    
}