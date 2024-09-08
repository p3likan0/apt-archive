use utoipa::OpenApi;
use crate::repo::{configuration::Configuration, Repository};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::repo::routes::handler_get_configuration,
        crate::repo::routes::handler_get_repositories,
        crate::repo::routes::handler_post_publish_repositories,
    ),
    components(
        schemas(Configuration, Repository),
    ),
)]
pub struct ApiDoc;
