use std::env;
use log::{error, info};
use std::process::{Command, exit};

fn is_work_dir() -> bool {
    let work_dir = "sevencode";

    match env::current_dir() {
        Ok(current_dir) => {
            current_dir.components()
                .any(|value| value.as_os_str().eq(work_dir))
        }
        Err(_) => { false }
    }
}

fn initialize_repo() {
   match Command::new("git")
       .arg("init")
       .spawn() {
       Ok(_) => {}
       Err(_) => {
           error!("Failed to initialize git repository!");
           exit(1)
       }
   }
}

fn git_config(key: &str, value: &str) {
    match Command::new("git")
        .arg("config")
        .arg(key)
        .arg(value)
        .spawn() {
        Ok(_) => {}
        Err(_) => {
            error!("Failed to set git config {}!", key);
            exit(1)
        }
    }
}

struct GitConfig {
    name: &'static str,
    email: &'static str,
}

pub fn init() {
    info!("Initializing git repository...");
    // TODO: This command isn't awaited
    initialize_repo();

    let work_config = GitConfig {
        name: "mihaisevencode",
        email: "mihai.miuta@7code.ro"
    };

    let personal_config = GitConfig {
        name: "miutamihai",
        email: "miuta.mihai@gmail.com"
    };

    if is_work_dir() {
        info!("Using work credentials");
        git_config("user.name", work_config.name);
        git_config("user.email", work_config.email);
    } else {
        info!("Using personal credentials");
        git_config("user.name", personal_config.name);
        git_config("user.email", personal_config.email);
    }
}
