
#[derive(thiserror::Error, Debug)]
pub enum RepoError {
    #[error("Could not write config to file, error:{0}")]
    CouldNotWriteConfigToFile(#[from] std::io::Error),

    #[error("Could not serialize config, error:{0}")]
    CouldNotSerializeConfigFile(#[from] toml::ser::Error),
}
pub type Result<T, E = RepoError> = std::result::Result<T, E>;