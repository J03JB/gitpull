use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Frees3c")]
pub struct ReposArgs {
    /// Add repo to list ( ~/.repos )
    pub add: String,
    /// Remove repo from list (~/.repos )
    pub delete: String,
    /// Pull all repos in list
    pub pull: String,
}
