use std::env;
use log::{error, info};
use std::process::exit;
use crate::common::run::{Input, run};
use crate::config::model::Config;
use crate::embedded::settings::get::get;


fn is_work_dir(config: &Config) -> bool {
    let work_dir = &config.work_dir;

    match env::current_dir() {
        Ok(current_dir) => {
            current_dir.components()
                .any(|value| value.as_os_str().to_str().unwrap().eq(work_dir))
        }
        Err(_) => { false }
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

struct GitConfig {
    name: &'static str,
    email: &'static str,
}

fn validate_config(config: &Config) {
    let config_settings = get().config;

    if config.work_dir == config_settings.default_value {
        error!("`work_dir` value not set!");
        error!("Please run: `mm config --name work_dir --value <your_work_dir>");
        exit(1)
    }
}

pub fn init(config: &Config) {
    info!("Initializing git repository...");
    validate_config(&config);
    initialize_repo();

    let work_config = GitConfig {
        name: "mihaisevencode",
        email: "mihai.miuta@7code.ro",
    };

    let personal_config = GitConfig {
        name: "miutamihai",
        email: "miuta.mihai@gmail.com",
    };

    if is_work_dir(config) {
        info!("Using work credentials");
        git_config("user.name", work_config.name);
        git_config("user.email", work_config.email);
    } else {
        info!("Using personal credentials");
        git_config("user.name", personal_config.name);
        git_config("user.email", personal_config.email);
    }
}
