use log::error;

pub fn validate(condition: bool, message: String) {
    if !condition {
        error!("{}", message);
        std::process::exit(1);
    }
}