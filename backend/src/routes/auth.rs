use anyhow::anyhow;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::{AppResult, AppState, models, respond_with_error};
use crate::models::User;

#[derive(Deserialize)]
pub struct JsonLogin {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct JsonResponse {
    username: String,
    sessionToken: String,
}

#[axum::debug_handler]
pub async fn login(State(state): State<AppState>, Json(credentials): Json<JsonLogin>) -> AppResult<Response> {
    if credentials.username.is_empty() || credentials.password.is_empty() {
        return respond_with_error(StatusCode::BAD_REQUEST, "empty_fields");
    }

    let Some(db_user) = models::get_user(&state.db, &credentials.username).await? else {
        return respond_with_error(StatusCode::BAD_REQUEST, "invalid_login");
    };

    if !bcrypt::verify(credentials.password, &db_user.password_hash).map_err(|e| anyhow!(e))? {
        return respond_with_error(StatusCode::BAD_REQUEST, "invalid_login");
    }

    Ok((StatusCode::OK, Json(JsonResponse { username: db_user.name })).into_response())
}

pub async fn register(State(state): State<AppState>, Json(credentials): Json<JsonLogin>) -> AppResult<Response> {
    if let Some(_) = models::get_user(&state.db, &credentials.username).await? {
        return respond_with_error(StatusCode::BAD_REQUEST, "username_taken");
    }

    let user = User {
        name: credentials.username,
        password_hash: bcrypt::hash(credentials.password, bcrypt::DEFAULT_COST).map_err(|e| anyhow!(e))?,
    };

    models::create_user(&state.db, &user).await?;

    Ok((StatusCode::OK, Json(JsonResponse { username: user.name })).into_response())
}
