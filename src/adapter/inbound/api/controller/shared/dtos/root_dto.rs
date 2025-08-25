use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiV1Endpoints {
    #[schema(value_type = String, example = "/api/v1/info")]
    pub info: String,
}

#[derive(Serialize, ToSchema)]
pub struct Endpoints {
    #[schema(value_type = String, example = "/health")]
    pub health: String,
    #[schema(value_type = ApiV1Endpoints)]
    pub api_v1: ApiV1Endpoints,
}

#[derive(Serialize, ToSchema)]
pub struct RootResponse {
    #[schema(value_type = String, example = "service name")]
    pub service: String,
    #[schema(value_type = String, example = "Rust Clean-Architecture starter with Axum")]
    pub description: String,
    #[schema(value_type = Endpoints)]
    pub endpoints: Endpoints,
}
