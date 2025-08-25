use axum::response::{IntoResponse, Json};

use crate::adapter::inbound::api::controller::shared::dtos::root_dto::{
    ApiV1Endpoints,
    Endpoints,
    RootResponse,
};

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Get service info", body = RootResponse)
    )
)]

pub async fn root() -> impl IntoResponse {
    let response = RootResponse {
        service: "eventories".to_string(),
        description: "Rust Clean-Architecture starter with Axum".to_string(),
        endpoints: Endpoints {
            health: "/health".to_string(),
            api_v1: ApiV1Endpoints {
                info: "/api/v1/info".to_string(),
            },
        },
    };
    Json(response)
}
