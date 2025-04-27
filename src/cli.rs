use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ya0201cli", version, about = "My Awesome CLI Toolchain")]
pub struct Cli {
    #[command(subcommand)]
    pub command: TopLevelCommand,
}

#[derive(Subcommand, Debug)]
pub enum TopLevelCommand {
    #[command(subcommand)]
    Repo(RepoCommand),

    #[command(subcommand)]
    Issue(IssueCommand),

    #[command(subcommand)]
    Subnet(SubnetCommand),

    #[command(name = "json2yaml")]
    Json2Yaml {
        #[arg(
            short,
            long,
            help = "the JSON file to convert to YAML.
            If not provided, reads from stdin"
        )]
        file: Option<String>,
    },

    #[command(name = "yaml2json")]
    Yaml2Json {
        #[arg(
            short,
            long,
            help = "the YAML file to convert to JSON.
            If not provided, reads from stdin"
        )]
        file: Option<String>,
    },
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

#[derive(Subcommand, Debug)]
pub enum SubnetCommand {
    Info {
        #[arg(help = "Subnet CIDR to get info about")]
        cidr: String,

        #[arg(short, long, help = "If specified, output will be JSON format")]
        json: bool,
    },
    // List {
    //     #[arg(help = "Subnet CIDR to list")]
    //     cidr: String,
    // },
}
