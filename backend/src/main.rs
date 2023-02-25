use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use axum::{Json, Router};
use axum::body::Body;
use axum::http::{HeaderMap, Method, Request, StatusCode};
use axum::http::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_REQUEST_METHOD};
use axum::middleware::Next;
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

pub struct AppStateStruct {
    db: Pool<Sqlite>,
}

pub type AppState = Arc<AppStateStruct>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy())
        .init();

    let sqlite_con = SqlitePool::connect_with(SqliteConnectOptions::from_str("sqlite://data.sqlite")?
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(true)).await?;


    sqlx::migrate!("./migrations")
        .run(&sqlite_con)
        .await?;

    let app_state = Arc::new(AppStateStruct {
        db: sqlite_con,
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", post(routes::login))
        .route("/register", post(routes::register));

    info!("Starting webserver...");

    let static_files_service = get_service(ServeDir::new("dist/")
        .append_index_html_on_directories(true)
        .fallback(ServeFile::new("dist/index.html")));

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(Router::new()
            .fallback(static_files_service)
            .nest("/api", app)
            .layer(axum::middleware::from_fn(|req: Request<Body>, next: Next<Body>| async {
                let mut cors_headers = HeaderMap::new();
                cors_headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse()
                    .map_err(|e| anyhow!("could not parse header {}", &e))?);
                cors_headers.insert(ACCESS_CONTROL_REQUEST_METHOD, "*".parse()
                    .map_err(|e| anyhow!("could not parse header {}", &e))?);
                cors_headers.insert(ACCESS_CONTROL_ALLOW_HEADERS, "*".parse()
                    .map_err(|e| anyhow!("could not parse header {}", &e))?);

                // Handle OPTIONS without reading body / etc.
                // While it does not check if the route even exists & co, this will allow CORS
                if req.method() == Method::OPTIONS {
                    return Ok::<_, AppError>((StatusCode::OK, cors_headers).into_response());
                }
                let mut res = next.run(req).await;
                cors_headers.into_iter().for_each(|(name, value)| { res.headers_mut().insert(name.unwrap(), value); });

                Ok::<_, AppError>(res)
            }))
            .with_state(app_state)

            .into_make_service())
        .await?;

    Ok(())
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    AnyhowError(anyhow::Error),
    Other(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = match self {
            AppError::AnyhowError(e) => {
                error!("got error in request: {}", &e);
                "internal_server_error".to_string()
            }
            AppError::Other(e) => e
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": &message}))).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self::AnyhowError(value)
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

impl std::error::Error for AppError {}

pub fn respond_with_error(status: StatusCode, err_msg: &str) -> AppResult<Response> {
    Ok((status, Json(json!({"error": err_msg}))).into_response())
}
