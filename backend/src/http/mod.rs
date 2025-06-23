use axum::{Json, Router, routing::get};
use serde::Serialize;
use tower_http::services::{ServeDir, ServeFile};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

pub fn new_router() -> Router {
    let serve_dir = ServeDir::new("public").not_found_service(ServeFile::new("public/index.html"));

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/api", get(health_check))
        .split_for_parts();

    router
        .merge(SwaggerUi::new("/api/swagger-ui").url("/api/openapi.json", api))
        .fallback_service(serve_dir)
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
