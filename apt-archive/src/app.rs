use std::sync::Arc;

use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{openapi::ApiDoc, repo::{self, configuration::Configuration, routes::{handler_get_configuration, handler_get_repositories}}, SharedState};


pub fn create_app(config: &Configuration) -> Router {
    let shared_state = Arc::new(SharedState::new(config.clone()));
    let openapi = ApiDoc::openapi();
    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", openapi);
    Router::new()
        .merge(repo::routes::create_routes(shared_state.clone()))
        .merge(swagger_ui)
}

