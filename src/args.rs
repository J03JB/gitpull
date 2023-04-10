use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Frees3c")]
pub struct ReposArgs {
    /// Add repo to list ( ~/.repos )
    pub add: Option<String>,
    /// Remove repo from list (~/.repos )
    pub delete: Option<String>,
    /// Pull all repos in list
    pub pull: Option<String>,
}
