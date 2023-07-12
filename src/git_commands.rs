use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum GitCommands {
    Init,
    PR
}
