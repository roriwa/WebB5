use axum::{Router, ServiceExt};
use axum::routing::{get, get_service};
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy())
        .init();


    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    info!("Starting webserver...");

    let static_files_service = get_service(ServeDir::new("dist/")
        .append_index_html_on_directories(true)
        .fallback(ServeFile::new("dist/index.html")));

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(Router::new()
            .fallback(static_files_service)
            .nest("/api", app)
            .into_make_service())
        .await?;

    Ok(())
}
