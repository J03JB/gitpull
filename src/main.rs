mod args;
mod git;
mod repos;

use crate::{
    args::ReposArgs,
    git::git_pull_with_threads,
    repos::{add_repo, del_repo, list_repos},
};

use clap::Parser;

const GR_FILE_PATH: &str = concat!(env!("HOME"), "/.repos");

pub fn main() {
    let args = ReposArgs::parse();

    match args {
        ReposArgs { pull_all: true, .. } => {
            println!("Pulling from all repositories ...\n");
            git_pull_with_threads(None);
        }
        ReposArgs {
            pull: Some(repo_name),
            ..
        } => {
            println!("Pulling from {}", repo_name);
            git_pull_with_threads(Some(&repo_name));
        }
        ReposArgs {
            add: Some(repo), ..
        } => {
            add_repo(repo).unwrap();
        }
        ReposArgs {
            delete: Some(repo), ..
        } => {
            if let Err(e) = del_repo(&repo) {
                eprintln!("{}", e);
                std::process::exit(1);
            } else {
                println!("{} has been removed from {}", repo, GR_FILE_PATH)
            }
        }
        ReposArgs { list: true, .. } => {
            println!("Tracked Repositories: \n");
            list_repos();
        }
        _ => {
            eprint!("No command specified");
            std::process::exit(1);
        }
    }
}
