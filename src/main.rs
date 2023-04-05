use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

fn main() {
    let repos = File::open("gr.txt").unwrap();
    let repo = BufReader::new(repos);

    for line in repo.lines() {
        let gitrepo = line.unwrap();
        gitpull(&gitrepo);
    }
}

// TODO: make function to add repo to gr.txt via --add.
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
