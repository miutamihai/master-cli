use log::error;
use std::process::exit;

pub fn error_handler(message: &String) {
    error!("{}", message);
    exit(1)
}
