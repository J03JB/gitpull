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
    if let Some(repo) = opts.delete {
        del_repo(repo);
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
    let repo = if repo == "." {
        let cwd = std::env::current_dir().unwrap();
        cwd.to_str().unwrap().to_owned()
    } else {
        repo
    };
    writeln!(file, "{}", repo).expect("failed to write to input file");
    println!("Added '{}' to input file", repo);
}

// TODO: make "." possible, same as above.
fn del_repo(repo: String) {
    let input_file = "gr.txt";
    let temp_file = "temp.txt";

    let mut input = File::open(input_file).expect("failed to open input file");
    let mut temp = File::create(temp_file).expect("failed to create temp file");

    let mut found = false;
    let mut line = String::new();

    // read input file line by line
    let mut reader = BufReader::new(input);
    while reader.read_line(&mut line).unwrap() > 0 {
        // check if the line contains the repository to delete
        if line.trim() == repo {
            found = true;
        } else {
            // write the line to the temp file if it's not the repository to delete
            temp.write(line.as_bytes())
                .expect("failed to write to temp file");
        }
        line.clear();
    }

    if found {
        // replace the input file with the temp file
        std::fs::rename(temp_file, input_file).expect("failed to replace input file");
        println!("Removed '{}' from input file", repo);
    } else {
        // delete the temp file if the repository was not found
        std::fs::remove_file(temp_file).expect("failed to delete temp file");
        println!("'{}' not found in input file", repo);
    }
}
