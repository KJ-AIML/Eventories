use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::adapter::inbound::api::controller::shared::dtos::health_dto;
use crate::adapter::inbound::api::controller::shared::dtos::root_dto;
use crate::adapter::inbound::api::controller::shared::handlers;
use crate::adapter::inbound::api::controller::v1::dtos::info_dto;
use crate::adapter::inbound::api::controller::v1::handlers as v1_handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::root::root,
        handlers::health::health,
        v1_handlers::info::info,
    ),
    components(
        schemas(
            root_dto::RootResponse,
            root_dto::Endpoints,
            root_dto::ApiV1Endpoints,
            health_dto::HealthResponse,
            info_dto::InfoResponse
        )
    ),
    tags(
        (name = "eventories", description = "eventories service API")
    )
)]
pub struct ApiDoc;

pub fn scalar_ui() -> axum::Router {
    axum::Router::new().merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
}
