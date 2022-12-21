use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    NonZeroStatus(i32),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CommandError::")?;
        match self {
            CommandError::NonZeroStatus(status) => f.write_fmt(format_args!("NonZeroStatus({status})"))
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u32,
    pub ip: String,
    pub webhook_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RepositoryConfig {
    #[serde(rename = "ref")]
    pub git_ref: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeploymentConfig {
    pub context: String,
    pub pull: Option<PullConfig>,
    pub descriptions: DeploymentDescriptions,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PullConfig {
    pub command: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeploymentDescriptions {
    pub success: String,
    pub failed_pull: String,
    pub failed_build: String,
    pub unknown_error: String,
    pub display: DescriptionDisplay,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DescriptionDisplay {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "display_name")]
    DisplayName,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Command {
    pub id: String,
    pub display_name: Option<String>,
    pub command: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub repository: RepositoryConfig,
    pub deployment: DeploymentConfig,
    pub commands: Vec<Command>,
}