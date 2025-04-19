mod cli;
mod commands;
mod config;

use clap::Parser;
use cli::{Cli, TopLevelCommand};
use commands::{handle_issue_command, handle_repo_command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        TopLevelCommand::Repo(cmd) => handle_repo_command(cmd),
        TopLevelCommand::Issue(cmd) => handle_issue_command(cmd),
    }
}
