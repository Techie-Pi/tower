use std::fs;
use std::path::{Path};
use std::process::Command as StdCommand;
pub use defs::*;
use crate::utils::string_to_command;

mod defs;

pub fn read_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let raw_config = fs::read_to_string(path)?;
    parse_config(&raw_config)
}

pub fn parse_config(config: &str) -> anyhow::Result<Config> {
    Ok(toml::from_str(&config)?)
}

impl Default for PullConfig {
    fn default() -> Self {
        Self {
            command: "git pull".to_string(),
        }
    }
}

impl Command {
    pub fn display(&self, context: &Config) -> &str {
        match context.deployment.descriptions.display {
            DescriptionDisplay::Id => &self.id,
            DescriptionDisplay::DisplayName => &self.display_name.as_ref().unwrap_or_else(|| &self.id),
        }
    }

    pub fn command(&self) -> StdCommand {
        string_to_command(&self.command)
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let result = self
            .command()
            .output()?;

        if result.status.code() != Some(0) {
            return Err(CommandError::NonZeroStatus(result.status.code().unwrap()).into())
        }

        Ok(())
    }
}

impl PullConfig {
    pub fn command(&self) -> StdCommand {
        string_to_command(&self.command)
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let result = self
            .command()
            .output()?;

        if result.status.code() != Some(0) {
            return Err(CommandError::NonZeroStatus(result.status.code().unwrap()).into())
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_can_be_parsed() {
        let toml_str = r#"
            [server]
            port = 80
            ip = "127.0.0.1"
            webhook_path = "/webhook/tower"

            [repository]
            ref = "refs/heads/main" # To prevent deployment of PRs and such

            [deployment]
            context = "Gazania Deployment"

            [deployment.pull] # OPTIONAL. This is the default value
            command = "git pull"

            [deployment.descriptions]
            success = "Deployment has been a success!"
            failed_pull = "Failed to perform a git pull"
            failed_build = "Failed to build the app; failed step {step}"
            unknown_error = "Unknown error"
            display = "display_name" # Can be ``id`` or ``display_name``

            [[commands]]
            id = "build"
            display_name = "Build"
            command = "cargo build --release"

            [[commands]]
            id = "deploy"
            display_name = "PM2 Restart"
            command = "pm2 restart 0"
        "#;

        let _config = parse_config(toml_str).expect("failed to parse config");
    }
}