use axum::response::{IntoResponse, Json};

use crate::adapter::inbound::api::controller::v1::dtos::info_dto::InfoResponse;

#[utoipa::path(
    get,
    path = "/api/v1/info",
    responses(
        (status = 200, description = "Get service info", body = InfoResponse)
    )
)]

pub async fn info() -> impl IntoResponse {
    let response = InfoResponse {
        service: "eventories".to_string(),
        api_version: "1".to_string(),
        status: "running".to_string(),
        message: "Welcome to eventories API v1".to_string(),
    };
    Json(response)
}
