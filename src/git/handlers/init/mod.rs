use crate::config::model::Config;
use crate::git::handlers::init::build_ssh_config::build_ssh_config;
use crate::git::handlers::init::git_config::git_config;
use crate::git::handlers::init::initialize_repo::initialize_repo;
use crate::git::handlers::init::is_work_dir::is_work_dir;
use crate::git::handlers::init::validate_config::validate_config;
use log::info;

mod build_ssh_config;
mod git_config;
mod initialize_repo;
mod is_work_dir;
mod validate_config;

pub fn init(config: Config) {
    validate_config(&config);
    info!(
        "Building work ssh config: {}",
        build_ssh_config(&config.git.work_credentials.ssh_key)
    );
    info!(
        "Building personal ssh config: {}",
        build_ssh_config(&config.git.personal_credentials.ssh_key)
    );

    if is_work_dir(&config) {
        info!("Initializing git repository...");
        initialize_repo();
        info!("Using work credentials");
        git_config("user.name", &config.git.work_credentials.name);
        git_config("user.email", &config.git.work_credentials.email);
    } else {
        info!("Initializing git repository...");
        initialize_repo();
        info!("Using personal credentials");
        git_config("user.name", &config.git.personal_credentials.name);
        git_config("user.email", &config.git.personal_credentials.email);
    }
}
