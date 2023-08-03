use crate::common::run::Input;

pub fn build_input(args: Vec<String>) -> Input {
    Input {
        cmd: String::from("git"),
        args,
        on_done: None,
        on_error: None,
    }
}

#[cfg(test)]
mod tests {
    use crate::common::run::Input;
    use crate::common::str_vec_to_string_vec::str_vec_to_string_vec;
    use crate::git::handlers::restart::build_input::build_input;

    #[test]
    fn build_input_test() {
        let args = str_vec_to_string_vec(
            vec!["checkout", "master"]
        );

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
