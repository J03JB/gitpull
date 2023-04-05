// TODO: add_repo(): if inside git dir, add it to list of repos.
use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::Command;

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Frees3c")]
struct Opts {
    #[clap(short, long, value_name = "add directory to list")]
    add: Option<String>,
    #[clap(short, long, value_name = "directory to remove from list")]
    delete: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    // add repository to list
    if let Some(repo) = opts.add {
        add_repo(repo);
        return;
    }

    let repos = File::open("gr.txt").unwrap();
    let repo = BufReader::new(repos);

    // pull repos listed in gr.txt
    for line in repo.lines() {
        let gitrepo = line.unwrap();
        gitpull(&gitrepo);
    }
}

// TODO: make function to remove repo from list --remove

fn gitpull(gitrepo: &str) {
    let mut command = Command::new("git");
    command.arg("-C").arg(gitrepo).arg("pull");

    let output = command.output().expect("failed to pull");
    if !output.status.success() {
        eprintln!("Error: Git pull failed for directory '{}'", gitrepo);
        eprintln!("Git: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Git pull succeeded for directory '{}'", gitrepo);
        // print output to console using stdout
        println!("Git: {}", String::from_utf8_lossy(&output.stdout));
    }
}

fn add_repo(repo: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("gr.txt")
        .expect("failed to open input file");
    writeln!(file, "{}", repo).expect("failed to write to input file");
    println!("Added '{}' to input file", repo);
}
