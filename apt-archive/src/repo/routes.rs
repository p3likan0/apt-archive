use std::sync::Arc;

use axum::{extract::State, Json};

use crate::SharedState;

use super::configuration::Configuration;

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