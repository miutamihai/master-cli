use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Initialize a new repo, with credentials based on the current profile")]
    Init,
    #[command(about = "Restart destination branch from origin")]
    Restart {
        #[arg(short, long)]
        origin: String,

        #[arg(short, long)]
        destination: String,
    },
    #[command(about = "Rebase destination branch from origin")]
    Rebase {
        #[arg(short, long)]
        origin: String,

        #[arg(short, long)]
        destination: String,
    },
}
