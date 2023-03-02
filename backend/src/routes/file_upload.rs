use std::collections::HashMap;
use std::io::SeekFrom;

use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use futures::TryStreamExt;
use serde_json::json;
use tempfile::tempfile;
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncWriteExt, BufWriter};
use tokio::sync::Mutex;
use tokio_util::io::StreamReader;
use tracing::info;
use uuid::Uuid;

use crate::{AppError, AppResult, AppState, AuthenticatedUser, respond_with_error};

pub async fn upload(
    State(state): State<AppState>,
    _require_auth: AuthenticatedUser,
    mut multipart: Multipart,
) -> AppResult {
    let Some(field) = multipart.next_field().await? else { return respond_with_error(StatusCode::BAD_REQUEST, "file_missing"); };
    let mapped_field = field.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));

    let file = tokio::fs::File::from(tempfile()?);

    let mut reader = StreamReader::new(mapped_field);
    let mut writer = BufWriter::new(file);
    let wrote = tokio::io::copy(&mut reader, &mut writer).await?;
    writer.flush().await?;

    info!("wrote file with {} bytes", wrote);

    if wrote == 0 {
        return Err(AppError::Other("not_written_anything".to_string()));
    }

    let mut file = writer.into_inner();
    file.sync_data().await?;
    file.seek(SeekFrom::Start(0)).await?;

    let key = state.file_upload.insert(file).await;

    Ok((StatusCode::OK, Json(json!({"key": &key.to_string()}))).into_response())
}

#[derive(Default)]
pub struct FileUploadStore {
    keys: Mutex<HashMap<String, File>>,
}

impl FileUploadStore {
    pub fn new() -> Self {
        Self {
            keys: Default::default(),
        }
    }

    pub async fn insert(&self, file: File) -> Uuid {
        let key = Uuid::new_v4();

        self.keys.lock().await.insert(key.to_string(), file);

        key
    }

    pub async fn retrieve(&self, key: String) -> Option<File> {
        self.keys.lock().await.remove(&key)
    }
}
