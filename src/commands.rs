pub mod repo;
pub mod issue;

use crate::cli::{RepoCommand, IssueCommand};

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
