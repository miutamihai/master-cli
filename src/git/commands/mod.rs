use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    PR,
    Restart {
        #[arg(short, long)]
        origin: String,

        #[arg(short, long)]
        destination: String
    },
}
