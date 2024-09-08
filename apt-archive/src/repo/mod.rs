use std::path::Path;

use debian_packaging::repository::builder::RepositoryBuilder;
use serde::{Deserialize, Serialize};

pub(crate) mod configuration;
mod error;
pub (crate) mod routes;
use configuration::Configuration;
use error::Result;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
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


// Implement axum, create endpoint for returning the configuration.
// Implement endpoint for returing repositories.