use crate::common::exit_with_errors::exit_with_errors;
use crate::config::handle::in_editor::in_editor;
use crate::config::handle::inline::inline;
use crate::config::model::Config;

mod in_editor;
mod inline;

fn build_message(missing: &'static str, existing: &'static str) -> String {
    format!(
        "Argument `--{}` is required when `--{}` is passed",
        missing, existing
    )
}

pub fn handle(name_option: &Option<String>, value_option: &Option<String>, config: Config) {
    match (name_option, value_option) {
        (None, None) => in_editor(),
        (Some(name), Some(value)) => inline(name, value, config),
        (None, Some(_)) => exit_with_errors(build_message("name", "value")),
        (Some(_), None) => exit_with_errors(build_message("value", "name")),
    }
}
