use log::{info};
use crate::common::run::{Input, run};

fn string(val: &str) -> String {
    val.to_string()
}

fn get_args(destination: String, origin: String) -> Vec<Vec<String>> {
    vec![
        vec![
            string("checkout"),
            origin
        ],
        vec![
            string("pull"),
            string("origin"),
            origin
        ],
        vec![
            string("branch"),
            string("-D"),
            destination
        ],
        vec![
            string("checkout"),
            string("-b"),
            destination
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

pub fn restart() {
    info!("What's your destination branch?");
    let mut destination = String::new();
    let mut origin = String::new();
    std::io::stdin().read_line(&mut destination).unwrap();
    info!("What's your origin branch?");
    std::io::stdin().read_line(&mut origin).unwrap();

    get_commands(destination, origin)
        .into_iter()
        .for_each(run)
}