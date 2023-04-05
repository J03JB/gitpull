use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let repos = File::open("gr.txt").unwrap();
    let repo = BufReader::new(repos);

    for line in repo.lines() {
        let directory = line.unwrap();
        // Do something with the directory here, such as passing it as an argument to a function
        println!("Processing directory: {}", directory);
        let args: Vec<String> = env::args().collect();
        println!("Program name: {}", args[0]);
        println!("Directory argument: {}", directory);
    }
}
