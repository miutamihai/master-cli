mod args;

use clap::Parser;
use args::Args;

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}

