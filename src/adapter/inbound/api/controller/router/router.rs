use axum::{routing::get, Router};

use crate::adapter::inbound::api::controller::shared::handlers::{root, health};
use crate::adapter::inbound::api::controller::v1;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root::root))
        .route("/health", get(health::health))
        .nest("/api/v1", v1::router::routes())
}
