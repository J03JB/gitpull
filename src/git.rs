use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

use crate::GR_FILE_PATH;

pub fn git_pull(repo_name: Option<&str>) {
    let repos_file = File::open(GR_FILE_PATH).expect(&format!("Failed to open '{}'", GR_FILE_PATH));
    let repos = BufReader::new(repos_file);

    let mut threads = vec![];
    let stdout_mutex = Arc::new(Mutex::new(std::io::stdout()));

    for line in repos.lines() {
        let gitrepo = line.expect(&format!("Failed to read line from '{}'", GR_FILE_PATH));
        if repo_name.is_none() || gitrepo.trim().ends_with(repo_name.unwrap()) {
            let stdout_mutex_clone = stdout_mutex.clone();
            let thread = thread::spawn(move || {
                // println!("Pulling from {} ...\n", gitrepo);
                let pull = Command::new("git")
                    .arg("-C")
                    .arg(&gitrepo)
                    .arg("-c")
                    .arg("color.ui=always")
                    .arg("pull")
                    .output()
                    .expect("failed to execute git");

                let mut stdout = stdout_mutex_clone.lock().unwrap();
                if pull.status.success() {
                    writeln!(&mut *stdout, "Pull succeeded for '{}'", gitrepo).unwrap();
                    writeln!(
                        &mut *stdout,
                        "Git: {}",
                        String::from_utf8_lossy(&pull.stdout)
                    )
                    .unwrap();
                } else {
                    writeln!(&mut *stdout, "Error: Pull failed for '{}'", gitrepo).unwrap();
                    writeln!(
                        &mut *stdout,
                        "Git: {}",
                        String::from_utf8_lossy(&pull.stderr)
                    )
                    .unwrap();
                }
            });

            threads.push(thread);
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
