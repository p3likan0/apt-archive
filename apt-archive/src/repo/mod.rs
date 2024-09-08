use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub(crate) mod configuration;
mod error;
pub (crate) mod routes;
pub (crate) mod publicate;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, ToSchema, derive_more::Display)]
#[display(fmt = "{:#?}", self)]
pub struct Repository {
    pub name: String,
    pub architectures: Vec<String>,
    pub components: Vec<String>,
    pub suite: String,
    pub codename: String,
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

