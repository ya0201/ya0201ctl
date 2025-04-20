pub mod issue;
pub mod json2yaml;
pub mod repo;

use crate::cli::{IssueCommand, RepoCommand};

pub fn handle_repo_command(cmd: RepoCommand) {
    match cmd {
        RepoCommand::Clone { repo } => repo::clone::run(repo),
        RepoCommand::List => repo::list::run(),
    }
}

pub fn handle_issue_command(cmd: IssueCommand) {
    match cmd {
        IssueCommand::Create { title } => issue::create::run(title),
        IssueCommand::List => issue::list::run(),
    }
}

pub fn handle_json2yaml_command(file: Option<String>) {
    json2yaml::run(file);
}
