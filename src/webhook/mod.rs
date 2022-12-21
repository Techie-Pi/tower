use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub date: Option<String>,
    pub email: Option<String>,
    pub name: String,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Committer {
    pub date: Option<String>,
    pub email: Option<String>,
    pub name: String,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub added: Option<Vec<String>>,
    pub author: Author,
    pub committer: Committer,
    pub distinct: bool,
    pub id: String,
    pub modified: Option<Vec<String>>,
    pub removed: Option<Vec<String>>,
    pub timestamp: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pusher {
    pub date: Option<String>,
    pub email: Option<String>,
    pub name: String,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushEvent {
    pub after: String,
    pub base_ref: Option<String>,
    pub before: String,
    pub commits: Vec<Commit>,
    pub compare: String,
    pub created: bool,
    pub deleted: bool,
    pub enterprise: Option<Value>,
    pub forced: bool,
    pub head_commit: Option<Commit>,
    pub installation: Option<Value>,
    pub organization: Option<Value>,
    pub pusher: Pusher,
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub repository: Repository,
    pub sender: Option<Value>,
}