use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct InfoResponse {
    #[schema(value_type = String, example = "service name")]
    pub service: String,
    #[schema(value_type = String, example = "v1")]
    pub api_version: String,
    #[schema(value_type = String, example = "running")]
    pub status: String,
    #[schema(value_type = String, example = "Welcome to service name API v1")]
    pub message: String,
}
