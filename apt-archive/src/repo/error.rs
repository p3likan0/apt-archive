use axum::{http::StatusCode, response::{IntoResponse, Response}};

use super::{configuration::Configuration, Repository};


#[derive(thiserror::Error, Debug)]
pub enum RepoError {
    #[error("Could not write config to file, error:{0}")]
    CouldNotWriteConfigToFile(#[from] std::io::Error),

    #[error("Could not serialize config, error:{0}")]
    CouldNotSerializeConfigFile(#[from] toml::ser::Error),

    #[error("Empty architecture for repo: {0}")]
    EmptyArchitecture(Repository),

    #[error("Empty component for repo: {0}")]
    EmptyComponent(Repository),

    #[error("Repository:{0} not present in config: {1}")]
    RepositoryNotPresentInConfiguration(Repository, Configuration),

    #[error("Debian error: {0}")]
    DebianError(#[from] debian_packaging::error::DebianError),
}
impl IntoResponse for RepoError {
    fn into_response(self) -> Response {
        let status_code = match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = self.to_string();
        (status_code, body).into_response()
    }
}
pub type Result<T, E = RepoError> = std::result::Result<T, E>;