use crate::common::run::Input;

pub fn build_input(args: Vec<String>) -> Input {
    Input {
        cmd: String::from("git"),
        args,
        on_done: None,
        on_error: None
    }
}

#[cfg(test)]
mod tests {
    use crate::common::run::Input;
    use crate::git::handlers::restart::build_input::build_input;

    fn get_args() -> Vec<String> {
        vec!["checkout", "master"]
            .iter()
            .map(|arg| {String::from(*arg)})
            .collect()
    }

    #[test]
    fn build_input_test() {
        let args = get_args();
        let expected = Input {
            cmd: String::from("git"),
            args: args.clone(),
            on_done: None,
            on_error: None
        };

        let actual = build_input(args);

        assert_eq!(expected.cmd, actual.cmd);
        assert_eq!(expected.args, actual.args);
        assert_eq!(expected.on_done, actual.on_done);
        assert_eq!(expected.on_error, actual.on_error);
    }

}
