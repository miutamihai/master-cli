use log::{info};
use crate::common::run::{Input, run};

fn string(val: &str) -> String {
    val.to_string()
}

fn get_args(destination: String, origin: String) -> Vec<Vec<String>> {
    vec![
        vec![
            string("checkout"),
            origin.clone()
        ],
        vec![
            string("pull"),
            string("origin"),
            origin.clone()
        ],
        vec![
            string("branch"),
            string("-D"),
            destination.clone()
        ],
        vec![
            string("checkout"),
            string("-b"),
            destination.clone()
        ]
    ]
}

fn build_input(args: Vec<String>) -> Input {
    Input {
        cmd: string("git"),
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