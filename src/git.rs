use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::Command,
    thread,
};

use crate::GR_FILE_PATH;

pub fn git_pull(repo_name: Option<&str>) {
    let repos_file = File::open(GR_FILE_PATH).expect(&format!("Failed to open '{}'", GR_FILE_PATH));
    let repos = BufReader::new(repos_file);
    // vector to hold the threads
    let mut threads = Vec::new();

    for line in repos.lines() {
        let gitrepo = line.expect(&format!("Failed to read line from '{}'", GR_FILE_PATH));
        if repo_name.is_none() || gitrepo.trim().ends_with(repo_name.unwrap()) {
            // spawn a new thread for each repository
            let thread_me_daddy = thread::spawn(move || {
                // println!("Pulling from {} ...\n", gitrepo);
                let pull = Command::new("git")
                    .arg("-C")
                    .arg(&gitrepo)
                    .arg("-c")
                    .arg("color.ui=always")
                    .arg("pull")
                    .output()
                    .expect("failed to execute git");

                if pull.status.success() {
                    println!("Git pull succeeded for '{}'", gitrepo);
                    println!("Git: {}", String::from_utf8_lossy(&pull.stdout));
                } else {
                    eprintln!("Error: Git pull failed for '{}'", gitrepo);
                    eprintln!("Git: {}", String::from_utf8_lossy(&pull.stderr));
                }
            });

            // add the thread to the vcector
            threads.push(thread_me_daddy);
        }
    }

    // wait for all threads to finsih
    for thread_me_daddy in threads {
        thread_me_daddy.join().unwrap();
    }
}
