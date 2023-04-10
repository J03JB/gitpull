use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1",
    author = "Frees3c",
    about = "A program to manage and pull from a list of git repositories."
)]
#[command(arg_required_else_help = true)]
pub struct ReposArgs {
    /// Add repo to list ( ~/.repos )
    #[clap(short, long)]
    pub add: Option<String>,
    /// Remove repo from list (~/.repos )
    #[clap(short, long)]
    pub delete: Option<String>,
    /// Pull all repos in list
    #[clap(short, long)]
    pub pull: bool,
}