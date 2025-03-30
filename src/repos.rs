pub use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
};

use crate::GR_FILE_PATH;

// TODO: abrv path to ~/
pub fn add_repo(repo: String) -> std::io::Result<()> {
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
    writeln!(file?, "{}", repo.trim_end_matches("/"))?;
    println!("'{}' was added to {}", repo, GR_FILE_PATH);
    Ok(())
}

pub fn del_repo(repo: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(GR_FILE_PATH)?;

    let contents = {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents
    };

    if !contents.lines().any(|line| line.trim().ends_with(&repo)) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            [repo, "not found in ", GR_FILE_PATH].join(" "),
        ));
    }

    let new_contents = contents
        .lines()
        .filter(|line| !line.trim().ends_with(&repo))
        .collect::<Vec<_>>()
        .join("\n");

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(new_contents.as_bytes())?;
    file.write_all("\n".as_bytes())?;
    file.flush()?;

    Ok(())
}

pub fn list_repos() {
    let repos_file = match Result::ok(File::open(GR_FILE_PATH)) {
        Some(file) => file,
        None => {
            println!("Error: File Not Found, {} ", GR_FILE_PATH);
            return;
        }
    };
    let repos = BufReader::new(repos_file);

    for lines in repos.lines() {
        println!("{}", lines.unwrap());
    }
}
