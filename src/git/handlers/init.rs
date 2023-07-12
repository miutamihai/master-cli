use std::env;
use log::info;

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

pub fn init() {
    info!("Initializing git repository...");

    if is_work_dir() {
        info!("Using work credentials")
    } else {
        info!("Using personal credentials")
    }
}
