use std::sync::Arc;

use axum::{extract::State, routing::{get, post}, Json, Router};

use crate::SharedState;

use super::{configuration::Configuration, publicate::publicate_repositories, Repository};

#[utoipa::path(
    get,
    path = "/v1/config",
    responses(
        (status = 200, description = "Repository configuration", body = [Configuration])
    )
)]
pub async fn handler_get_configuration(
    State(shared_state): State<Arc<SharedState>>,
) -> Result<Json<Configuration>, super::error::RepoError> {
    Ok(Json(shared_state.config.clone()))
}

#[utoipa::path(
    get,
    path = "/v1/repositories",
    responses(
        (status = 200, description = "Repositories", body = [Vec<Repository>])
    )
)]
pub async fn handler_get_repositories(
    State(shared_state): State<Arc<SharedState>>,
) -> Result<Json<Vec<Repository>>, super::error::RepoError> {
    Ok(Json(shared_state.config.repositories.clone()))
}

#[utoipa::path(
    post,
    path = "/v1/repositories",
    responses(
        (status = 200, description = "Repositories")
    )
)]
async fn handler_post_publish_repositories(
    State(shared_state): State<Arc<SharedState>>,
    Json(repositories): Json<Vec<Repository>>,
) -> Result<(), super::error::RepoError> {
    //shared_state.config.validate_repositories_exists(&repositories)?;
    publicate_repositories(&repositories, &shared_state.config).await?;
    Ok(())
}

pub fn create_routes(state: Arc<SharedState>) -> Router {
    Router::new()
        .route("/v1/config", get(handler_get_configuration))
        .route("/v1/repositories", get(handler_get_repositories))
        .route( "/v1/repositories", post(handler_post_publish_repositories))
        .with_state(state)
}
