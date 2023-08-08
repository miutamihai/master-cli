use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about="Initialize a new repo, with credentials based on the work folder")]
    Init,
    #[command(about="Create a new Pull Request")]
    PR,
    #[command(about="Restart destination branch from origin")]
    Restart {
        #[arg(short, long)]
        origin: String,

        #[arg(short, long)]
        destination: String
    },
}
