use crate::config::model::Config;
use crate::git::handlers::init::git_config::git_config;
use crate::git::handlers::init::initialize_repo::initialize_repo;
use crate::git::handlers::init::is_work_dir::is_work_dir;
use crate::git::handlers::init::validate_config::validate_config;
use log::info;

mod error_handler;
mod git_config;
mod initialize_repo;
mod is_work_dir;
mod validate_config;

pub fn init(config: Config) {
    info!("Initializing git repository...");
    validate_config(&config);
    initialize_repo();

    if is_work_dir(&config) {
        info!("Using work credentials");
        git_config("user.name", &config.git.work_credentials.name);
        git_config("user.email", &config.git.work_credentials.email);
    } else {
        info!("Using personal credentials");
        git_config("user.name", &config.git.personal_credentials.name);
        git_config("user.email", &config.git.personal_credentials.email);
    }
}