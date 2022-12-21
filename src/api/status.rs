use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde_json::Value;
use crate::api::base_api;

pub enum CommitState {
    Error,
    Failure,
    Pending,
    Success,
}

impl Display for CommitState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitState::Error => f.write_str("error"),
            CommitState::Failure => f.write_str("failure"),
            CommitState::Pending => f.write_str("pending"),
            CommitState::Success => f.write_str("success"),
        }
    }
}

pub struct CommitStatusData {
    pub state: CommitState,
    pub target_url: Option<String>,
    pub description: Option<String>,
    pub context: Option<String>,
}

pub fn create_commit_status<'a>(owner: &'a str, repo: &'a str, sha: &'a str, data: CommitStatusData) -> anyhow::Result<Value> {
    let mut hashmap_data = HashMap::new();

    hashmap_data.insert("state", data.state.to_string());

    if let Some(target_url) = data.target_url {
        hashmap_data.insert("target_url", target_url);
    }

    if let Some(description) = data.description {
        hashmap_data.insert("description", description);
    }

    if let Some(context) = data.context {
        hashmap_data.insert("context", context);
    }

    base_api::<Value>(&format!("/repos/{owner}/{repo}/statuses/{sha}"), hashmap_data)
}