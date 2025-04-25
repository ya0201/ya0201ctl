mod cli;
mod commands;
mod config;

use clap::Parser;
use cli::{Cli, TopLevelCommand};
use commands::{
    handle_issue_command, handle_json2yaml_command, handle_repo_command, handle_subnet_command,
    handle_yaml2json_command,
};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        TopLevelCommand::Repo(cmd) => handle_repo_command(cmd),
        TopLevelCommand::Issue(cmd) => handle_issue_command(cmd),
        TopLevelCommand::Subnet(cmd) => handle_subnet_command(cmd),
        TopLevelCommand::Json2Yaml { file } => handle_json2yaml_command(file),
        TopLevelCommand::Yaml2Json { file } => handle_yaml2json_command(file),
    }
}
