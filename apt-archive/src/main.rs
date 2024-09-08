use std::path::Path;

use app::create_app;
mod app;
mod openapi;
mod repo;
use debian_packaging::repository::{builder::{RepositoryBuilder, NO_PROGRESS_CB, NO_SIGNING_KEY}, filesystem::{FilesystemRepositoryReader, FilesystemRepositoryWriter}};
use repo::configuration::Configuration;

pub struct SharedState {
    pub config: Configuration,
    pub writer: FilesystemRepositoryWriter,
    pub resolver: FilesystemRepositoryReader,
}
impl SharedState {
    pub fn new(config: Configuration) -> Self {
        SharedState { config,  writer: FilesystemRepositoryWriter::new("/tmp/test"), resolver: FilesystemRepositoryReader::new("/tmp/test")}
    }
}
#[tokio::main]
async fn main() {
    let config = Configuration::from_read_or_create_config_file(Path::new("config.toml")).unwrap();
    let app = create_app(&config);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", &config.server_ip, &config.server_port))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}