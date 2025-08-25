use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    #[schema(value_type = String, example = "healthy")]
    pub status: String,
    #[schema(value_type = String, example = "service name")]
    pub service: String,
    #[schema(value_type = String, format = DateTime, example = "2025-08-01T12:00:00Z")]
    pub timestamp: DateTime<Utc>,
    #[schema(value_type = String, example = "running")]
    pub uptime: String,
}
