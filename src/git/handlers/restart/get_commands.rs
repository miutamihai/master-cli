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
    use crate::common::str_vec_to_string_vec::str_vec_to_string_vec;
    use crate::git::handlers::restart::get_commands::get_commands;

    #[test]
    fn get_commands_test() {
        let origin = "origin";
        let destination = "destination";

        let expected = vec![
            Input {
                cmd: String::from("git"),
                args: str_vec_to_string_vec(vec!["checkout", origin.clone()]),
                on_done: None,
                on_error: None,
            },
            Input {
                cmd: String::from("git"),
                args: str_vec_to_string_vec(vec!["pull", "origin", origin.clone()]),
                on_done: None,
                on_error: None,
            },
            Input {
                cmd: String::from("git"),
                args: str_vec_to_string_vec(vec!["branch", "-D", destination.clone()]),
                on_done: None,
                on_error: None,
            },
            Input {
                cmd: String::from("git"),
                args: str_vec_to_string_vec(vec!["checkout", "-b", destination.clone()]),
                on_done: None,
                on_error: None,
            },
        ];

        let actual = get_commands(&String::from(destination), &String::from(origin));

        assert_eq!(expected, actual)
    }
}
