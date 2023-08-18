use crate::config::model::Config;
use crate::git::handlers::init::git_config::git_config;
use crate::git::handlers::init::initialize_repo::initialize_repo;
use crate::git::handlers::init::is_work_dir::is_work_dir;
use crate::git::handlers::init::ssh_config::setup;
use crate::git::handlers::init::validate_config::validate_config;
use log::info;

mod git_config;
mod initialize_repo;
mod is_work_dir;
mod ssh_config;
mod validate_config;

pub fn init(config: Config) {
    validate_config(&config);

    // TODO: Remove this once the Profiles feature is implemented
    if is_work_dir(&config) {
        info!("Initializing git repository...");
        initialize_repo();
        info!("Using work credentials");
        let ssh_config_path = setup(&config.git.work_credentials.ssh_key);
        git_config("user.name", &config.git.work_credentials.name);
        git_config("user.email", &config.git.work_credentials.email);
        git_config(
            "core.sshCommand",
            &format!("ssh -F \"{}\"", ssh_config_path),
        );
    } else {
        info!("Initializing git repository...");
        initialize_repo();
        info!("Using personal credentials");
        let ssh_config_path = setup(&config.git.personal_credentials.ssh_key);
        git_config("user.name", &config.git.personal_credentials.name);
        git_config("user.email", &config.git.personal_credentials.email);
        git_config(
            "core.sshCommand",
            &format!("ssh -F \"{}\"", ssh_config_path),
        );
    }
}
