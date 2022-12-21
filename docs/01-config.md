# Configuration

Tower config is in TOML format, a full example of the config follows:

```toml
[server]
port = 80
ip = "127.0.0.1"
webhook_path = "/webhook/tower"
reload_config_on_deployment = true # This is NOT able to change things like port or ip on the fly!

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
```