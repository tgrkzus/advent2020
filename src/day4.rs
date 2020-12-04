use super::*;
use std::collections::HashMap;

// all fields: ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
pub fn run(input: &Vec<String>) {
    first(input);
    second(input);
}

fn first(input: &Vec<String>) {
    let passports = parse(input);
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let result = passports.iter()
        .filter(|passport| {
            required.iter().fold(true, |acc, key| acc && passport.contains_key(key))
        })
        .count();
    println!("{}", result);
}

fn second(input: &Vec<String>) {
    let passports = parse(input);
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let result = passports.iter()
        .filter(|passport| {
            required.iter().fold(true, |acc, key| acc && passport.contains_key(key))
        })
        .filter(|passport| {
            passport.iter().fold(true, |acc, (key, value)| acc && validate(key, value))
        })
        .count();
    println!("{}", result);
}

fn validate(key: &str, val: &str) -> bool {
    return match key {
        "byr" => {
            let as_num = val.parse::<i32>().unwrap();
            as_num >= 1920 && as_num <= 2002
        },
        "iyr" => {
            let as_num = val.parse::<i32>().unwrap();
            as_num >= 2010 && as_num <= 2020
        },
        "eyr" => {
            let as_num = val.parse::<i32>().unwrap();
            as_num >= 2020 && as_num <= 2030
        },
        "hgt" => {
            if let Ok(as_num) = val.chars().take(val.len() - 2).collect::<String>().parse::<i32>() {
                return if val.ends_with("cm") {
                    as_num >= 150 && as_num <= 193
                } else if val.ends_with("in") {
                    as_num >= 59 && as_num <= 76
                } else {
                    false
                };
            }
            return false;
        },
        "hcl" => {
            let regex = Regex::new("#([a-fA-F0-9]){6}").unwrap();
            regex.is_match(val)
        },
        "ecl" => {
            let could_be = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            could_be.iter().any(|potential| val.eq(*potential))
        },
        "pid" => {
            val.parse::<i32>().is_ok() && val.len() == 9
        },
        _ => true
    }
}

fn parse(input: &Vec<String>) -> Vec<HashMap<&str, &str>> {
    let mut passports = Vec::new();
    let mut current = HashMap::new();
    for line in input {
        if line.is_empty() {
            let old = current;
            current = HashMap::new();
            passports.push(old);
            continue;
        }
        line.split(" ")
            .map(|pair| {
                let mut pair_split = pair.split(":");
                (pair_split.next().unwrap(), pair_split.next().unwrap())
            })
            .for_each(|(key, value)| { current.insert(key, value); })
    }
    passports.push(current);
    passports
}


