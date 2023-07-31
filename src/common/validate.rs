use log::error;

pub fn validate(condition: bool, message: &str) {
    if !condition {
        error!("{}", message);
        std::process::exit(1);
    }
}