use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ya0201cli", version, about = "My Awesome CLI Toolchain")]
pub struct Cli {
    #[command(subcommand)]
    pub command: TopLevelCommand,
}

#[derive(Subcommand, Debug)]
pub enum TopLevelCommand {
    Repo(RepoCommand),
    Issue(IssueCommand),
}

#[derive(Subcommand, Debug)]
pub enum RepoCommand {
    Clone {
        #[arg(help = "Repository to clone")]
        repo: String,
    },
    List,
}

#[derive(Subcommand, Debug)]
pub enum IssueCommand {
    Create {
        #[arg(help = "Issue title")]
        title: String,
    },
    List,
}
