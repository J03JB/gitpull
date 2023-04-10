mod args;

use args::ReposArgs;
use clap::Parser;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
    process::Command,
};

// TODO: switch from deprecated env. to 'dirs' or 'directories'
const GR_FILE_PATH: &str = concat!(env!("HOME"), "/.repos");

pub fn main() {
    let args = ReposArgs::parse();

    let repos = File::open(GR_FILE_PATH)
        .expect(&format!("No Git repositories found in '{}'", GR_FILE_PATH));
    let repo = BufReader::new(repos);

    for line in repo.lines() {
        let gitrepo = line.expect(&format!("Failed to read line from '{}'", GR_FILE_PATH));
        if args.pull {
            println!("Pulling from {} ...\n", gitrepo);
            git_pull(&gitrepo);
        }
    }

    if let Some(repo) = args.add {
        add_repo(repo).unwrap();
    } else if let Some(repo) = args.delete {
        del_repo(&repo).unwrap();
    } else {
        eprint!("No command specified.");
        std::process::exit(1);
    }
}

pub fn git_pull(gitrepo: &str) {
    let output = Command::new("git")
        .arg("-C")
        .arg(gitrepo)
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

// TODO: abrv path to ~/
fn add_repo(repo: String) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(GR_FILE_PATH);
    let repo = if repo == "." {
        let cwd = std::env::current_dir()?;
        cwd.to_str().unwrap().to_owned()
    } else {
        repo
    };
    writeln!(file?, "{}", repo)?;
    println!("'{}' was added to {}", repo, GR_FILE_PATH);
    Ok(())
}

fn del_repo(repo: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(GR_FILE_PATH)
        .expect("failed to open input file");

    let contents = {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("failed to read input file");
        contents
    };

    let new_contents = contents
        .lines()
        .filter(|line| !line.trim().ends_with(&repo))
        .collect::<Vec<_>>()
        .join("\n");

    file.set_len(0).expect("failed to truncate input file");
    file.seek(SeekFrom::Start(0))
        .expect("failed to seek to start of input file");
    file.write_all(new_contents.as_bytes())
        .expect("failed to write to input file");
    file.write_all("\n".as_bytes())
        .expect("Failed to write to input file");
    file.flush().expect("failed to flush");

    println!("Removed '{}' from input file", repo);
    Ok(())
}

// #[test]
// fn testing() {
//     let gr_file_path = home_dir().unwrap().join(".repos");
//     println!("{}", gr_file_path);
// }
