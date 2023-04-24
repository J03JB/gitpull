use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::Command,
};

use crate::GR_FILE_PATH;

pub fn git_pull(repo_name: Option<&str>) {
    let repos_file = File::open(GR_FILE_PATH).expect(&format!("Failed to open '{}'", GR_FILE_PATH));
    let repos = BufReader::new(repos_file);

    for line in repos.lines() {
        let gitrepo = line.expect(&format!("Failed to read line from '{}'", GR_FILE_PATH));
        if repo_name.is_none() || gitrepo.trim().ends_with(repo_name.unwrap()) {
            println!("Pulling from {} ...\n", gitrepo);
            let output = Command::new("git")
                .arg("-C")
                .arg(&gitrepo)
                .arg("-c")
                .arg("color.ui=always")
                .arg("pull")
                .output()
                .expect("failed to execute git");

            if output.status.success() {
                println!("Git pull succeeded for '{}'", gitrepo);
                println!("Git: {}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("Error: Git pull failed for '{}'", gitrepo);
                eprintln!("Git: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }
}
