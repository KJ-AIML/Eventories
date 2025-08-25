use std::net::SocketAddr;
use axum::Router;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod adapter;
mod application;
mod domain;
mod infrastructure;
mod shared;

use adapter::inbound::api::controller::router;
use adapter::inbound::api::controller::docs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting {} service...", "eventories");
    
    // Load configuration
    let config = infrastructure::config::load_config();
    tracing::info!("Environment: {}", config.environment);
    tracing::info!("Log level: {}", config.log_level);
    
    // Create the main router
    let app = create_app();

    // Run our application
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn create_app() -> Router {
    Router::new()
        .merge(router::router::routes())
        .merge(docs::scalar_ui())
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(axum::middleware::from_fn(shared::middleware::log_requests))
        .layer(TraceLayer::new_for_http())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Signal received, starting graceful shutdown");
}
