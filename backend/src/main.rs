use axum::Router;
use axum::routing::get;
use axum_extra::routing::SpaRouter;
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

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(Router::new()
            .merge(SpaRouter::new("/", "dist/"))
            .nest("/api", app)
            .into_make_service())
        .await?;

    Ok(())
}
