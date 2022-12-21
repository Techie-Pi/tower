use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::process::Command;
use serde::Deserialize;
use thiserror::Error;

pub mod status;

#[derive(Error, Debug)]
pub(self) enum ExecutionError {
    GitHubError(String),
    JsonParsingError(serde_json::Error),
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

pub(self) fn base_api<T>(endpoint: &str, payload: HashMap<&'static str, String>) -> anyhow::Result<T>
    where T: for <'a> Deserialize<'a>
{
    let mut command = Command::new("gh");
    let mut command = command
        .arg("api")
        .arg("--method")
        .arg("POST")
        .arg("-H")
        .arg("Accept: application/vnd.github+json")
        .arg(endpoint);

    for (name, value) in payload {
        command = command
            .arg("-f")
            .arg(format!("{name}={value}"));
    }

    let value = command.output()?;
    if value.stderr.len() != 0 {
        return Err(ExecutionError::GitHubError(String::from_utf8_lossy(&value.stderr).to_string()).into())
    }

    let response = String::from_utf8_lossy(&value.stdout);
    let result = serde_json::from_str::<T>(&response);
    if let Err(value) = result {
        return Err(ExecutionError::JsonParsingError(value).into())
    }

    Ok(result.ok().unwrap())
}