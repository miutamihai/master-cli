use crate::common::run::{run, Input};
use crate::config::model::Config;
use crate::config::validate::{validate, Rule};
use log::{error, info};
use std::env;
use std::process::exit;

fn is_work_dir(config: &Config) -> bool {
    let work_dir = &config.git.work_dir;

    match env::current_dir() {
        Ok(current_dir) => current_dir
            .components()
            .any(|value| value.as_os_str().to_str().unwrap().eq(work_dir)),
        Err(_) => false,
    }
}

fn error_handler(message: &String) {
    error!("{}", message);
    exit(1)
}

fn initialize_repo() {
    let input = Input {
        cmd: "git".to_string(),
        args: vec!["init".to_string()],
        on_done: None,
        on_error: Some(error_handler),
    };

    run(input).ok();
}

fn git_config(key: &str, value: &str) {
    let input = Input {
        cmd: "git".to_string(),
        args: vec!["config".to_string(), key.to_string(), value.to_string()],
        on_done: None,
        on_error: Some(error_handler),
    };

    run(input).ok();
}

fn validate_config(config: &Config) {
    validate(vec![
        Rule {
            name: "git.work_dir",
            value: config.git.work_dir.clone(),
        },
        Rule {
            name: "git.personal_credentials.name",
            value: config.git.personal_credentials.name.clone(),
        },
        Rule {
            name: "git.personal_credentials.email",
            value: config.git.personal_credentials.email.clone(),
        },
        Rule {
            name: "git.work_credentials.name",
            value: config.git.work_credentials.name.clone(),
        },
        Rule {
            name: "git.work_credentials.email",
            value: config.git.work_credentials.email.clone(),
        },
    ])
}

pub fn init(config: &Config) {
    info!("Initializing git repository...");
    validate_config(&config);
    initialize_repo();

    if is_work_dir(config) {
        info!("Using work credentials");
        git_config("user.name", &config.git.work_credentials.name);
        git_config("user.email", &config.git.work_credentials.email);
    } else {
        info!("Using personal credentials");
        git_config("user.name", &config.git.personal_credentials.name);
        git_config("user.email", &config.git.personal_credentials.email);
    }
}
