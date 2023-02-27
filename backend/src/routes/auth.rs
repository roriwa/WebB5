use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;

use crate::{AppResult, AppState, AuthenticatedUser, models, respond_with_error};
use crate::models::User;

#[derive(Deserialize)]
pub struct JsonLogin {
    username: String,
    password: String,
}

#[axum::debug_handler]
pub async fn login(State(state): State<AppState>, Json(credentials): Json<JsonLogin>) -> AppResult {
    let username = credentials.username.replace(" ", "");

    if username.is_empty() || credentials.password.is_empty() {
        return respond_with_error(StatusCode::BAD_REQUEST, "empty_fields");
    }

    let Some(db_user) = models::get_user(&state.db, &username).await? else {
        return respond_with_error(StatusCode::BAD_REQUEST, "invalid_login");
    };

    if !bcrypt::verify(credentials.password, &db_user.password_hash)? {
        return respond_with_error(StatusCode::BAD_REQUEST, "invalid_login");
    }

    let session = models::add_session(&state.db, &db_user).await?;

    Ok((StatusCode::OK, Json(session)).into_response())
}

pub async fn register(State(state): State<AppState>, Json(credentials): Json<JsonLogin>) -> AppResult {
    let username = credentials.username.replace(" ", "");

    if username.is_empty() || credentials.password.is_empty() {
        return respond_with_error(StatusCode::BAD_REQUEST, "empty_fields");
    }

    if username.len() > 16 || credentials.password.len() < 8 {
        return respond_with_error(StatusCode::BAD_REQUEST, "invalid_requirements");
    }

    if let Some(_) = models::get_user(&state.db, &username).await? {
        return respond_with_error(StatusCode::BAD_REQUEST, "username_taken");
    }

    let user = User {
        name: username,
        password_hash: bcrypt::hash(credentials.password, bcrypt::DEFAULT_COST)?,
    };

    models::create_user(&state.db, &user).await?;

    let session = models::add_session(&state.db, &user).await?;

    Ok((StatusCode::OK, Json(session)).into_response())
}

pub async fn logout(State(state): State<AppState>, user: AuthenticatedUser) -> AppResult {
    models::delete_session(&state.db, &user.session_token).await?;

    Ok((StatusCode::OK, Json(json!({"ok": true}))).into_response())
}

pub async fn whois(user: AuthenticatedUser) -> AppResult {
    Ok((StatusCode::OK, Json(json!({"username": user.username}))).into_response())
}
