use std::path::Path;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::error::RepoError::{
    EmptyArchitecture, EmptyComponent, RepositoryNotPresentInConfiguration,
};

use super::error::Result;
use super::Repository;

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema, derive_more::Display)]
#[display(fmt = "{:#?}", self)]
pub struct Configuration {
    pub repositories: Vec<Repository>,
    pub repo_root_path: String,
    pub server_ip: String,
    pub server_port: u16,
}

impl Configuration {
    fn default() -> Self {
        Configuration {
            repositories: vec![Repository::default()],
            server_ip: "0.0.0.0".to_owned(),
            server_port: 3000,
            repo_root_path: "".to_owned(),
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

    pub fn validate_repositories_exists(&self, repos: &Vec<Repository>) -> Result<()> {
        for repo in repos {
            if repo.architectures.is_empty() {
                return Err(EmptyArchitecture(repo.clone()));
            }
            if repo.components.is_empty() {
                return Err(EmptyComponent(repo.clone()));
            }
            if self
                .repositories
                .iter()
                .find(|r| r.name == repo.name)
                .is_none()
            {
                return Err(RepositoryNotPresentInConfiguration(
                    repo.clone(),
                    self.clone(),
                ));
            }
        }
        Ok(())
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
    #[test]
    fn test_validate_repositories_exists_valid() {
        let repo1 = Repository {
            name: "repo1".to_owned(),
            architectures: vec!["amd64".to_owned()],
            components: vec!["main".to_owned()],
            suite: "stable".to_owned(),
            codename: "buster".to_owned(),
        };
        let repo2 = Repository {
            name: "repo2".to_owned(),
            architectures: vec!["arm64".to_owned()],
            components: vec!["contrib".to_owned()],
            suite: "unstable".to_owned(),
            codename: "bullseye".to_owned(),
        };
        let config = Configuration {
            repositories: vec![repo1.clone(), repo2.clone()],
            server_ip: "0.0.0.0".to_owned(),
            server_port: 3000,
            repo_root_path: "".to_owned(),
        };
        let repos = vec![repo1, repo2];
        let result = config.validate_repositories_exists(&repos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_repositories_exists_empty_architecture() {
        let repo1 = Repository {
            name: "repo1".to_owned(),
            architectures: vec![],
            components: vec!["main".to_owned()],
            suite: "stable".to_owned(),
            codename: "buster".to_owned(),
        };
        let config = Configuration {
            repositories: vec![repo1.clone()],
            server_ip: "0.0.0.0".to_owned(),
            server_port: 3000,
            repo_root_path: "".to_owned(),
        };
        let repos = vec![repo1];
        let result = config.validate_repositories_exists(&repos);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_repositories_exists_empty_component() {
        let repo1 = Repository {
            name: "repo1".to_owned(),
            architectures: vec!["amd64".to_owned()],
            components: vec![],
            suite: "stable".to_owned(),
            codename: "buster".to_owned(),
        };
        let config = Configuration {
            repositories: vec![repo1.clone()],
            server_ip: "0.0.0.0".to_owned(),
            server_port: 3000,
            repo_root_path: "".to_owned(),
        };
        let repos = vec![repo1];
        let result = config.validate_repositories_exists(&repos);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_repositories_exists_repository_not_present() {
        let repo1 = Repository {
            name: "repo1".to_owned(),
            architectures: vec!["amd64".to_owned()],
            components: vec!["main".to_owned()],
            suite: "stable".to_owned(),
            codename: "buster".to_owned(),
        };
        let repo2 = Repository {
            name: "repo2".to_owned(),
            architectures: vec!["arm64".to_owned()],
            components: vec!["contrib".to_owned()],
            suite: "unstable".to_owned(),
            codename: "bullseye".to_owned(),
        };
        let config = Configuration {
            repositories: vec![repo1.clone()],
            server_ip: "0.0.0.0".to_owned(),
            server_port: 3000,
            repo_root_path: "".to_owned(),
        };
        let repos = vec![repo2];
        let result = config.validate_repositories_exists(&repos);
        assert!(result.is_err());
    }
}
