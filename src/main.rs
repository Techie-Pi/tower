use femme::LevelFilter;
use tide::log::{error, info};
use tide::{Request, Response, StatusCode};
use crate::api::status::{CommitState, CommitStatusData, create_commit_status};
use crate::config::{Config, read_config};
use crate::deploy::deploy_app;
use crate::webhook::PushEvent;

mod config;
mod api;
mod webhook;
mod deploy;
mod utils;

#[async_std::main]
async fn main() {
    femme::with_level(LevelFilter::Info);

    info!("[Tower] Version {}", env!("CARGO_PKG_VERSION"));

    let config = read_config("tower.toml").expect("failed to read config");

    let listen_str = format!("{}:{}", config.server.ip.clone(), config.server.port.clone());
    let webhook_path = config.server.webhook_path.clone();

    let mut app = tide::with_state(config);
    app.at(&webhook_path).post(|mut req: Request<Config>| async move {
        let value: PushEvent = req.body_json().await?;
        let config = req.state();

        if value.git_ref != config.repository.git_ref {
            return Ok(Response::new(StatusCode::Ok))
        }

        let repo_name = value.repository.full_name.split("/").collect::<Vec<&str>>();
        if let Err(e) = deploy_app(repo_name.get(0).unwrap(), repo_name.get(1).unwrap(), &value.after, &config) {
            error!("{:?}", e);
            create_commit_status(repo_name.get(0).unwrap(), repo_name.get(1).unwrap(), &value.after, CommitStatusData {
                state: CommitState::Error,
                target_url: None,
                description: Some(config.deployment.descriptions.unknown_error.clone()),
                context: Some(config.deployment.context.clone()),
            })?;
        }

        Ok(Response::new(StatusCode::Ok))
    });

    app.listen(listen_str).await.expect("failed to start server");
}
