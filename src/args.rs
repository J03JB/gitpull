use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.2",
    author = "Frees3c",
    about = "A program to manage and pull from a list of git repositories."
)]
#[command(arg_required_else_help = true)]
pub struct ReposArgs {
    /// Add repo to list ( ~/.repos )
    #[clap(short, long)]
    pub add: Option<String>,
    /// Remove repo from list (~/.repos )
    #[clap(short = 'r', long)]
    pub delete: Option<String>,
    /// List repos in ( ~/.repos )
    #[clap(short, long)]
    pub list: bool,
    /// Pull selected repo from list ( ~/.repos )
    #[clap(short, long)]
    pub pull: Option<String>,
    /// Pull from all repos
    #[clap(short = 'x', long)]
    pub pull_all: bool,
}
