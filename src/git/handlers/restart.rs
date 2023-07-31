use log::{info};
use crate::common::run::{Input, run};

fn get_args(destination: String, origin: String) -> Vec<Vec<String>> {
    vec![
        vec![
            String::from("checkout"),
            origin.clone()
        ],
        vec![
            String::from("pull"),
            String::from("origin"),
            origin.clone()
        ],
        vec![
            String::from("branch"),
            String::from("-D"),
            destination.clone()
        ],
        vec![
            String::from("checkout"),
            String::from("-b"),
            destination.clone()
        ]
    ]
}

fn build_input(args: Vec<String>) -> Input {
    Input {
        cmd: String::from("git"),
        args,
        on_done: None,
        on_error: None
    }
}

fn get_commands(destination: String, origin: String) -> Vec<Input> {
    get_args(destination, origin)
        .into_iter()
        .map(build_input)
        .collect()
}

fn get_input(message: &str) -> String {
    info!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn restart() {
    let destination = get_input("What's your destination branch?");
    let origin = get_input("What's your origin branch?");

    get_commands(destination, origin)
        .into_iter()
        .flat_map(run)
        .collect()
}