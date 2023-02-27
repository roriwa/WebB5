use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use axum::{async_trait, Json, Router};
use axum::extract::{DefaultBodyLimit, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, get_service, post};
use serde_json::json;
use sqlx::{Pool, Sqlite, SqlitePool};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use tower_http::services::{ServeDir, ServeFile};
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod routes;
pub mod models;

pub const FILE_UPLOAD_PATH: &str = "images/";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy())
        .init();

    let sqlite_con = SqlitePool::connect_with(SqliteConnectOptions::from_str("sqlite://./data.sqlite")?
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(true)).await?;


    sqlx::migrate!("./migrations")
        .run(&sqlite_con)
        .await?;

    tokio::fs::create_dir_all(FILE_UPLOAD_PATH).await?;

    let app_state = Arc::new(AppStateStruct {
        db: sqlite_con,
        file_upload: routes::file_upload::FileUploadStore::new(),
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", post(routes::login))
        .route("/register", post(routes::register))
        .route("/logout", post(routes::logout))
        .route("/whois", post(routes::whois))
        .route("/upload", post(routes::file_upload::upload))
        .route("/add_recipe", post(routes::add_recipe))
        .route("/recipes", post(routes::recipes))
        .route("/bookmark", post(routes::bookmark))
        .route("/bookmarks", post(routes::get_bookmarks))
        .route("/image/:key", get(routes::get_recipe_image))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 10));

    info!("Starting webserver...");

    let static_files_service = get_service(ServeDir::new("dist/")
        .append_index_html_on_directories(true)
        .fallback(ServeFile::new("dist/index.html")));

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(Router::new()
            .fallback(static_files_service)
            .nest("/api", app)
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .layer(tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any))
            .with_state(app_state)

            .into_make_service())
        .await?;

    Ok(())
}

pub type AppState = Arc<AppStateStruct>;

pub struct AppStateStruct {
    db: Pool<Sqlite>,
    file_upload: routes::file_upload::FileUploadStore,
}

pub struct AuthenticatedUser {
    username: String,
    session_token: String,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let Some(auth_header) = parts.headers.get(axum::http::header::AUTHORIZATION.as_str())
            .into_iter()
            .flat_map(|header| header.to_str().ok())
            .flat_map(|bearer| bearer.strip_prefix("Bearer "))
            .last()
            else { return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "unauthorized"})))); };

        let Ok(session) = models::get_user_by_session(&state.db, auth_header).await
            else { return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"erorr": "could_not_get_session"})))); };

        match session {
            None => Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "unauthorized"})))),
            Some(u) => Ok(AuthenticatedUser {
                username: u.name,
                session_token: auth_header.to_string(),
            })
        }
    }
}

pub type AppResult = Result<Response, AppError>;

#[derive(Debug)]
pub enum AppError {
    AnyhowError(anyhow::Error),
    Other(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = match self {
            AppError::AnyhowError(e) => {
                error!("got error in request: {:?}", e);
                "internal_server_error".to_string()
            }
            AppError::Other(e) => e
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": &message}))).into_response()
    }
}

impl<E> From<E> for AppError where E: Into<anyhow::Error> {
    fn from(value: E) -> Self {
        Self::AnyhowError(value.into())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::AnyhowError(e) => write!(f, "AppError::AnyhowError: {e}"),
            AppError::Other(e) => write!(f, "AppError::Other: {e}")
        }
    }
}

pub fn respond_with_error(status: StatusCode, err_msg: &str) -> AppResult {
    Ok((status, Json(json!({"error": err_msg}))).into_response())
}
