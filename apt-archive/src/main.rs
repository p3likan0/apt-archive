use std::{path::Path, sync::Arc};

use axum::{extract::State, routing::get, Router};
use openapi::ApiDoc;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
mod repo;
mod openapi;
use repo::{configuration::Configuration, routes::handler_get_configuration};

pub struct SharedState {
    pub config: Configuration,
}
impl SharedState {
    pub fn new(config: Configuration) -> Self {
        SharedState { config }
    }
    
}

#[tokio::main]
async fn main() {
    let config = Configuration::from_read_or_create_config_file(Path::new("config.toml")).unwrap();
    let shared_state = Arc::new(SharedState::new(config.clone()));

    let app = Router::new().route("/v1/config", get(handler_get_configuration)).with_state(shared_state);

    let openapi = ApiDoc::openapi();
    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", openapi);
    let app = app.merge(swagger_ui);

    let listener =
    tokio::net::TcpListener::bind(format!("{}:{}", &config.server_ip, &config.server_port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

//pub fn create_repositories() -> Result<()> {
//    for repo in config.repositories {
//        let repo_builder = RepositoryBuilder::new_recommended(
//            repo.architectures.iter(),
//            repo.components.iter(),
//            &repo.suite,
//            &repo.codename,
//        );
//        print!("{:?}", repo_builder);
//    }
//    Ok(())
//}

