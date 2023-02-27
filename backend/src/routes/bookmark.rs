use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;

use crate::{AppResult, AppState, AuthenticatedUser, models};

#[derive(Deserialize)]
pub struct JsonBookmark {
    pub recipe_id: String,
    pub bookmarked: bool,
}

pub async fn bookmark(State(state): State<AppState>, user: AuthenticatedUser, Json(body): Json<JsonBookmark>) -> AppResult {
    models::bookmark(&state.db, user.username, body.recipe_id, body.bookmarked).await?;

    Ok((StatusCode::OK, Json(json!({}))).into_response())
}

pub async fn get_bookmarks(State(state): State<AppState>, user: AuthenticatedUser) -> AppResult {
    let bookmarks = models::get_all_bookmarks(&state.db, user.username).await?;

    Ok((StatusCode::OK, Json(json!({"bookmarks": bookmarks}))).into_response())
}
