use crate::common::run::Input;
use crate::git::handlers::restart::build_input::build_input;
use crate::git::handlers::restart::get_args::get_args;

pub fn get_commands(destination: &String, origin: &String) -> Vec<Input> {
    get_args(destination, origin)
        .into_iter()
        .map(build_input)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::common::run::Input;
    use crate::common::traits::as_string_vec::AsStringVec;
    use crate::git::handlers::restart::get_commands::get_commands;

    #[test]
    fn get_commands_test() {
        let origin = "origin";
        let destination = "destination";

        let expected = vec![
            Input {
                cmd: String::from("git"),
                args: vec!["checkout", origin].as_string_vec(),
                on_done: None,
                on_error: None,
            },
            Input {
                cmd: String::from("git"),
                args: vec!["pull", "origin", origin].as_string_vec(),
                on_done: None,
                on_error: None,
            },
            Input {
                cmd: String::from("git"),
                args: vec!["branch", "-D", destination].as_string_vec(),
                on_done: None,
                on_error: None,
            },
            Input {
                cmd: String::from("git"),
                args: vec!["checkout", "-b", destination].as_string_vec(),
                on_done: None,
                on_error: None,
            },
        ];

        let actual = get_commands(&String::from(destination), &String::from(origin));

        assert_eq!(expected, actual)
    }
}
