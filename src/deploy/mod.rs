use std::fmt::{Display, Formatter};
use crate::api::status::{CommitState, CommitStatusData, create_commit_status};
use crate::config::{Config, PullConfig};
use thiserror::Error;
use tide::log::info;

pub fn deploy_app(owner: &str, repo: &str, sha: &str, config: &Config) -> anyhow::Result<()> {
    let context = config.deployment.context.clone();
    create_commit_status(owner, repo, sha, CommitStatusData {
        state: CommitState::Pending,
        target_url: None,
        description: None,
        context: Some(context.clone()),
    })?;

    let default_pull_command = PullConfig {
        command: "git pull".to_owned(),
    };

    info!("Running git pull");
    if config.deployment.pull
        .as_ref()
        .unwrap_or(&default_pull_command)
        .run()
        .is_err()
    {
        create_commit_status(owner, repo, sha, CommitStatusData {
            state: CommitState::Error,
            target_url: None,
            description: Some(config.deployment.descriptions.failed_pull.to_owned()),
            context: Some(context.clone()),
        })?;

        return Ok(())
    }

    for command in &config.commands {
        info!("Running command: {}", command.display(&config));
        if command.run().is_err() {
            create_commit_status(owner, repo, sha, CommitStatusData {
                state: CommitState::Failure,
                target_url: None,
                description: Some(config.deployment.descriptions.failed_build.replace("{step}", command.display(config))),
                context: Some(context.clone()),
            })?;

            return Ok(())
        }
    }

    create_commit_status(owner, repo, sha, CommitStatusData {
        state: CommitState::Success,
        target_url: None,
        description: Some(config.deployment.descriptions.success.to_owned()),
        context: Some(context),
    })?;

    Ok(())
}