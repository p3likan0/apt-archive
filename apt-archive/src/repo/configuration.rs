use core::error;
use std::path::Path;
use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::SharedState;

use super::Repository;
use super::error::Result;

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct Configuration {
    pub repositories: Vec<Repository>,
    pub server_ip: String,
    pub server_port: u16,
}

impl Configuration {
    fn default() -> Self {
        Configuration {
            repositories: vec![Repository::default()],
            server_ip: "0.0.0.0".to_owned(),
            server_port: 3000,
        }
    }

    fn write_to_config_file(&self, config_path: &Path) -> Result<()> {
        let config = toml::to_string_pretty(&self)?;
        std::fs::write(config_path, config)?;
        Ok(())
    }

    pub fn from_read_or_create_config_file(config_path: &Path) -> Result<Self> {
        let config = match std::fs::read_to_string(config_path) {
            Ok(content) => content,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                let repos = Configuration::default();
                repos.write_to_config_file(config_path)?;
                return Ok(repos);
            }
            Err(err) => {
                panic!("Could not read config file: {}", err);
            }
        };
        let config: Configuration = toml::from_str(&config).expect("Could not parse config file");
        Ok(config)
    }
}



#[cfg(test)]
mod tests {
    use tempdir::TempDir;

    use super::*;

    #[test]
    fn test_default_repository() {
        let repository = Repository::default();
        assert_eq!(repository.name, "stable");
        assert_eq!(repository.architectures, vec!["amd64", "arm64"]);
        assert_eq!(repository.components, vec!["main", "contrib"]);
        assert_eq!(repository.suite, "stable");
        assert_eq!(repository.codename, "buster");
    }

    #[test]
    fn test_default_repositories() {
        let config = Configuration::default();
        let default_repo = Repository::default();
        assert_eq!(config.repositories.len(), 1);
        let repository = &config.repositories[0];
        assert_eq!(repository, &default_repo);
    }

    #[test]
    fn test_write_to_config_file() {
        let tmp_dir = TempDir::new("test").unwrap();
        let config = Configuration::default();
        let config_path = tmp_dir.path().join("test_config.toml");
        config
            .write_to_config_file(&config_path)
            .expect("Could not write config file");
        assert!(config_path.exists());
        std::fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_from_read_or_create_config_file_existing() {
        let config = Configuration::from_read_or_create_config_file(Path::new(
            "tests/assets/existing_config.toml",
        ))
        .unwrap();
        let default_repo = Repository::default();
        assert_eq!(config.repositories.len(), 1);
        let repository = &config.repositories[0];
        assert_ne!(repository, &default_repo);
        assert_eq!(repository.name, "unstable");
        assert_eq!(repository.architectures, vec!["amd64", "arm64"]);
        assert_eq!(repository.components, vec!["main", "contrib"]);
        assert_eq!(repository.suite, "unstable");
        assert_eq!(repository.codename, "buster");
    }

    #[test]
    fn test_from_read_or_create_config_file_non_existing() {
        let tmp_dir = TempDir::new("test").unwrap();
        let config_path = tmp_dir.path().join("non_existing_config.toml");
        let config = Configuration::from_read_or_create_config_file(&config_path).unwrap();
        let default_repo = Repository::default();
        assert_eq!(config.repositories.len(), 1);
        let repository = &config.repositories[0];
        assert_eq!(repository, &default_repo);
        assert!(config_path.exists());
        std::fs::remove_file(config_path).unwrap();
    }
}
