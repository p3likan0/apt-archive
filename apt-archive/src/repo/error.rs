use axum::{http::StatusCode, response::{IntoResponse, Response}};


#[derive(thiserror::Error, Debug)]
pub enum RepoError {
    #[error("Could not write config to file, error:{0}")]
    CouldNotWriteConfigToFile(#[from] std::io::Error),

    #[error("Could not serialize config, error:{0}")]
    CouldNotSerializeConfigFile(#[from] toml::ser::Error),
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