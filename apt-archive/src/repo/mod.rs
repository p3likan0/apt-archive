use std::path::Path;

use debian_packaging::repository::builder::RepositoryBuilder;
use serde::{Deserialize, Serialize};

mod configuration;
mod error;
use configuration::Configuration;
use error::Result;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
struct Repository {
    name: String,
    architectures: Vec<String>,
    components: Vec<String>,
    suite: String,
    codename: String,
}

impl Repository {
    pub fn default() -> Self {
        Repository {
            name: "stable".to_string(),
            architectures: vec!["amd64".to_string(), "arm64".to_string()],
            components: vec!["main".to_string(), "contrib".to_string()],
            suite: "stable".to_string(),
            codename: "buster".to_string(),
        }
    }
}

pub fn create_repositories() -> Result<()> {
    let config = Configuration::from_read_or_create_config_file(Path::new("config.toml"))?;
    for repo in config.repositories {
        let repo_builder = RepositoryBuilder::new_recommended(
            repo.architectures.iter(),
            repo.components.iter(),
            &repo.suite,
            &repo.codename,
        );
        print!("{:?}", repo_builder);
    }
    Ok(())
}
