mod args;
mod git;
mod repos;

use crate::args::ReposArgs;

use clap::Parser;

const GR_FILE_PATH: &str = concat!(env!("HOME"), "/.repos");

pub fn main() {
    let args = ReposArgs::parse();

    match args {
        ReposArgs { pull_all: true, .. } => {
            println!("\x1b[37;1mPulling from all repositories...\x1b[0m\n");
            git::git_pull(None);
        }
        ReposArgs {
            pull: Some(repo_name),
            ..
        } => {
            println!("Pulling from {}", repo_name);
            git::git_pull(Some(&repo_name));
        }
        ReposArgs {
            add: Some(repo), ..
        } => {
            repos::add_repo(repo).unwrap();
        }
        ReposArgs {
            delete: Some(repo), ..
        } => {
            if let Err(e) = repos::del_repo(&repo) {
                eprintln!("{}", e);
                std::process::exit(1);
            } else {
                println!("{} has been removed from {}", repo, GR_FILE_PATH)
            }
        }
        ReposArgs { list: true, .. } => {
            println!("Tracked Repositories: ");
            repos::list_repos();
        }
        _ => {
            eprint!("No command specified");
            std::process::exit(1);
        }
    }
}
