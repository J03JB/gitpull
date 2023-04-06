use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::SeekFrom;
use std::io::{BufRead, BufReader};
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};
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

    let gr_file_path = format!("{}/.repos", std::env::var("HOME").unwrap());

    // add repository to list
    if let Some(repo) = opts.add {
        add_repo(repo, &gr_file_path);
        return;
    }
    if let Some(repo) = opts.delete {
        del_repo(repo, &gr_file_path);
        return;
    }

    let repos = File::open(&gr_file_path).unwrap();
    let repo = BufReader::new(repos);

    // pull repos listed in gr.txt
    for line in repo.lines() {
        let gitrepo = line.unwrap();
        gitpull(&gitrepo);
    }
}

// TODO: use this inplace of 'let gr_file_path'
fn get_list_file_path(filename: &str) -> PathBuf {
    let home_dir = std::env::var("HOME").expect("Failed to get home directory.");
    let gr_path = Path::new(&home_dir).join(".repos");
    gr_path
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

fn add_repo(repo: String, gr_file_path: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(gr_file_path)
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
fn del_repo(repo: String, gr_file_path: &str) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(gr_file_path)
        .expect("failed to open input file");

    let repo = if repo == "." {
        match std::env::current_dir() {
            Ok(current_dir) => current_dir.display().to_string(),
            Err(e) => {
                eprintln!("Failed to get current directory: {}", e);
                return;
            }
        }
    } else {
        repo
    };

    let contents = {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("failed to read input file");
        contents
    };

    let new_contents = contents
        .lines()
        .filter(|line| *line != repo)
        .collect::<Vec<_>>()
        .join("\n");

    file.set_len(0).expect("failed to truncate input file");
    file.seek(SeekFrom::Start(0))
        .expect("failed to seek to start of input file");
    file.write_all(new_contents.as_bytes())
        .expect("failed to write to input file");

    println!("Removed '{}' from input file", repo);
}
// fn del_repo(repo: String, gr_file_path: &str) {
//     let input_file = gr_file_path;
//     let temp_file = "temp.txt";

//     let mut input = File::open(input_file).expect("failed to open input file");
//     let mut temp = File::create(temp_file).expect("failed to create temp file");

//     let mut found = false;
//     let mut line = String::new();

//     // read input file line by line
//     let mut reader = BufReader::new(input);
//     while reader.read_line(&mut line).unwrap() > 0 {
//         // check if the line contains the repository to delete
//         if line.trim() == repo {
//             found = true;
//         } else {
//             // write the line to the temp file if it's not the repository to delete
//             temp.write(line.as_bytes())
//                 .expect("failed to write to temp file");
//         }
//         line.clear();
//     }

//     if found {
//         // replace the input file with the temp file
//         std::fs::rename(temp_file, input_file).expect("failed to replace input file");
//         println!("Removed '{}' from input file", repo);
//     } else {
//         // delete the temp file if the repository was not found
//         std::fs::remove_file(temp_file).expect("failed to delete temp file");
//         println!("'{}' not found in input file", repo);
//     }
// }
