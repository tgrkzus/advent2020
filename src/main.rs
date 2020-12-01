use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use std::env;
use itertools::Itertools;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let problem: u32 = args[1].parse().unwrap();
            let file: String = args[1].parse().unwrap();
            run_problem(problem, file)
        }
        _ => {
            println!("Args wrong... ./advent <problem 1-n>")
        }
    }
}

fn run_problem(problem: u32, file: String) {
    println!("Running problem {} with file {}", problem, file);
    let input = get_input(&file);
    // TODO this could prob be some macro idk
    match problem {
        1 => {
            day1::run(&input)
        },
        _ => {
            println!("Problem {} not found.", problem)
        }
    }
}

fn get_input(file: &str) -> Vec<String> {
    let f = File::open(format!("inputs/{}", file)).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|r| r.expect("Failed to read line"))
        .collect()
}
