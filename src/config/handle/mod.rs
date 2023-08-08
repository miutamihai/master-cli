use log::{info};

pub fn handle(name: &String, value: &String) {
    info!("Setting config {} to {}", name, value);
}