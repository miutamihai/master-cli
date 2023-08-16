use crate::common::exit_with_errors::exit_with_errors;
use crate::common::run::Input;

pub fn build_input(args: Vec<String>) -> Input {
    Input {
        cmd: String::from("git"),
        args,
        on_done: None,
        on_error: Some(|error| exit_with_errors(error)),
    }
}

#[cfg(test)]
mod tests {
    use crate::common::run::Input;
    use crate::common::traits::as_string_vec::AsStringVec;
    use crate::git::handlers::restart::build_input::build_input;

    #[test]
    fn build_input_test() {
        let args = vec!["checkout", "master"].as_string_vec();

        let expected = Input {
            cmd: String::from("git"),
            args: args.clone(),
            on_done: None,
            on_error: None,
        };

        let actual = build_input(args);

        assert_eq!(expected, actual);
    }
}
