use axum::response::{IntoResponse, Json};
use chrono::Utc;

use crate::adapter::inbound::api::controller::shared::dtos::health_dto::HealthResponse;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Get service health", body = HealthResponse)
    )
)]
pub async fn health() -> impl IntoResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "eventories".to_string(),
        timestamp: Utc::now(),
        uptime: "running".to_string(),
    };
    Json(response)
}
