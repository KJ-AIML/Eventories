use axum::{routing::get, Router};
use crate::adapter::inbound::api::controller::v1::handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/info", get(handlers::info))
}
