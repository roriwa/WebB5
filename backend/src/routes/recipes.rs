use std::io::ErrorKind;

use axum::body::StreamBody;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;
use tokio::fs::File;
use tokio::io::{BufReader, BufWriter};
use uuid::Uuid;

use crate::{AppResult, AppState, AuthenticatedUser, FILE_UPLOAD_PATH, models, respond_with_error};
use crate::models::{Ingredient, Recipe};

#[derive(Deserialize)]
pub struct JsonAddRecipe {
    name: String,
    tags: Vec<String>,
    time_required: String,
    summary: String,
    description: String,
    ingredients: Vec<Ingredient>,
    upload_file_key: String,
}

pub async fn recipes(State(state): State<AppState>) -> AppResult {
    let recipes = models::get_all_recipes(&state.db).await?;

    Ok((StatusCode::OK, Json(json!({"recipes": recipes}))).into_response())
}

pub async fn add_recipe(State(state): State<AppState>, user: AuthenticatedUser, Json(recipe): Json<JsonAddRecipe>) -> AppResult {
    let name = recipe.name.replace(" ", "");
    if name.is_empty() {
        return respond_with_error(StatusCode::BAD_REQUEST, "invalid_name");
    }


    // Move tempfile into images/ dir
    let Some(image) = state.file_upload.retrieve(recipe.upload_file_key).await else { return respond_with_error(StatusCode::BAD_REQUEST, "no_image"); };
    let retrieve_key = Uuid::new_v4().to_string();
    let new_file = get_file(&retrieve_key, true).await?;

    let mut reader = BufReader::new(image);
    let mut writer = BufWriter::new(new_file);
    tokio::io::copy(&mut reader, &mut writer).await?;

    let recipe = Recipe {
        id: Uuid::new_v4().to_string(),
        name,
        author: user.username,
        tags: recipe.tags,
        time_required: recipe.time_required,
        summary: recipe.summary,
        description: recipe.description,
        image_key: retrieve_key,
        ingredients: recipe.ingredients,
        comments: vec![],
    };

    models::insert_recipe(&state.db, &recipe).await?;

    Ok((StatusCode::OK, Json(json!({"recipe": recipe}))).into_response())
}


pub async fn get_recipe_image(Path(key): Path<String>) -> AppResult {
    let file = match get_file(key, false).await {
        Ok(f) => f,
        Err(e) => return match e.kind() {
            ErrorKind::NotFound => respond_with_error(StatusCode::NOT_FOUND, "no_file"),
            _ => Err(e.into())
        }
    };


    let stream = tokio_util::io::ReaderStream::new(file);
    let body = StreamBody::new(stream);
    let header = axum::response::AppendHeaders([
        (axum::http::header::CONTENT_TYPE, "image/jpeg")
    ]);

    Ok((StatusCode::OK, header, body).into_response())
}

async fn get_file(key: impl AsRef<str>, write: bool) -> std::io::Result<File> {
    let key = key.as_ref().replace("/", "").replace("..", "");

    tokio::fs::OpenOptions::new()
        .create(write)
        .write(write)
        .read(!write)
        .open(format!("{}{}.jpg", FILE_UPLOAD_PATH, &key))
        .await
}
