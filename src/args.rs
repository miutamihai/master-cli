use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "mm")]
#[command(author = "Mihai Miuta <miuta.mihai@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "CLI to handle common tasks", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub name: String,

    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}
