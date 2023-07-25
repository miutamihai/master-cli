use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    PR,
    Restart,
}
