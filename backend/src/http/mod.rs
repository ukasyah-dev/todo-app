use axum::{Json, Router, routing::get};
use serde::Serialize;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

pub fn new_router() -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/api", get(health_check))
        .split_for_parts();

    router.merge(SwaggerUi::new("/api/swagger-ui").url("/api/openapi.json", api))
}

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
}

pub async fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: String::from("ok"),
    })
}
