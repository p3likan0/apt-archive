use utoipa::OpenApi;
use crate::repo::configuration::Configuration;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::repo::routes::handler_get_configuration,
    ),
    components(
        schemas(Configuration),
    ),
)]
pub struct ApiDoc;
