use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::process::Command;

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Frees3c")]
struct Opts {
    #[clap(short, long, value_name = "add directory to list")]
    add: Option<String>,
    #[clap(short, long, value_name = "directory to remove from list")]
    delete: Option<String>,
}

const GR_FILE_PATH: &str = concat!(env!("HOME"), "/.repos");

fn main() {
    let opts = Opts::parse();

    // add repository to list
    if let Some(repo) = opts.add {
        add_repo(repo).unwrap();
        return;
    }
    if let Some(repo) = opts.delete {
        del_repo(repo);
        return;
    }

    let repos = File::open(GR_FILE_PATH).unwrap_or_else(|err| {
        eprint!("No Git repositories found in '{}': {}\n", GR_FILE_PATH, err);
        eprint!(
            "Try running 'gpull --add .' from within a git repository, \n or 'gpull --add path/to/repo'"
        );
        std::process::exit(1);
    });
    let repo = BufReader::new(repos);

    // pull repos listed in gr.txt
    for line in repo.lines() {
        let gitrepo = line.unwrap_or_else(|err| {
            eprint!("Failed to read line from '{}': {}", GR_FILE_PATH, err);
            std::process::exit(1);
        });
        git_pull(&gitrepo);
    }
}
fn git_pull(gitrepo: &str) {
    let output = Command::new("git")
        .arg("-C")
        .arg(gitrepo)
        .arg("pull")
        .output()
        .expect("failed to execute git");

    if output.status.success() {
        println!("Git pull succeeded for directory '{}'", gitrepo);
        println!("Git: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Error: Git pull failed for directory '{}'", gitrepo);
        eprintln!("Git: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn add_repo(repo: String) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(GR_FILE_PATH);
    // let repo = repo.unwrap_or_else(|| std::env::current_dir().unwrap());
    let repo = if repo == "." {
        let cwd = std::env::current_dir()?;
        cwd.to_str().unwrap().to_owned()
    } else {
        repo
    };
    writeln!(file?, "{}", repo)?;
    println!("Added '{}' to input file", repo);
    Ok(())
}

fn del_repo(repo: String) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(GR_FILE_PATH)
        .expect("failed to open input file");

    let repo = match repo.as_ref() {
        "." => std::env::current_dir().unwrap().display().to_string(),
        _ => repo,
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
    file.flush().expect("failed to flush");

    println!("Removed '{}' from input file", repo);
}
