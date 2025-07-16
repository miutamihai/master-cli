use log::info;

use crate::config::current_profile::current_profile;
use crate::git::init::git_config::git_config;
use crate::git::init::initialize_repo::initialize_repo;
use crate::git::init::ssh_config::setup;

mod git_config;
mod initialize_repo;
mod ssh_config;

// TODO: Stop passing config everywhere
pub fn init() {
    let profile = current_profile();

    info!("Initializing git repository...");
    initialize_repo();
    let ssh_config_path = setup(&profile.git_credentials.ssh_key);
    git_config("user.name", &profile.git_credentials.name);
    git_config("user.email", &profile.git_credentials.email);
    git_config(
        "core.sshCommand",
        &format!("ssh -F \"{}\"", ssh_config_path),
    );
}
