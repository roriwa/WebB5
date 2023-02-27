use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;

use crate::{AppResult, AppState, AuthenticatedUser, models};

#[derive(Deserialize)]
pub struct JsonAddComment {
    recipe_id: String,
    comment: String,
}

pub async fn comment(State(state): State<AppState>, user: AuthenticatedUser, Json(body): Json<JsonAddComment>) -> AppResult {
    models::add_comment(&state.db, body.recipe_id, body.comment, user.username, Utc::now().timestamp_millis()).await?;

    Ok((StatusCode::OK, Json(json!({}))).into_response())
}
