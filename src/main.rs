#![feature(str_split_once)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::prelude::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

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
        2 => {
            day2::run(&input)
        },
        3 => {
            day3::run(&input)
        },
        4 => {
            day4::run(&input)
        },
        5 => {
            day5::run(&input)
        },
        6 => {
            day6::run(&input)
        },
        7 => {
            day7::run(&input)
        },
        8 => {
            day8::run(&input)
        },
        9 => {
            day9::run(&input)
        },
        10 => {
            day10::run(&input)
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
