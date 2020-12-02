use super::*;

pub fn run(input: &Vec<String>) {
    first(input);
    second(input);
}

fn first(input: &Vec<String>) {
    let result = input.iter()
        .filter(|line| {
            let mut split = line.split(" ");
            let (min, max) = {
                let mut range = split.next().unwrap().split("-");
                (
                    range.next().unwrap().parse::<usize>().unwrap(),
                    range.next().unwrap().parse::<usize>().unwrap()
                )
            };
            let character = split.next().unwrap().chars().nth(0).unwrap();
            let password = split.next().unwrap();

            let count = password.matches(character).count();
            return count <= max && count >= min;
        })
        .count();

    println!("{}", result);
}

fn second(input: &Vec<String>) {
    let result = input.iter()
        .filter(|line| {
            let mut split = line.split(" ");
            let (first, second) = {
                let mut range = split.next().unwrap().split("-");
                (
                    range.next().unwrap().parse::<usize>().unwrap(),
                    range.next().unwrap().parse::<usize>().unwrap()
                )
            };
            let character = split.next().unwrap().chars().nth(0).unwrap();
            let password = split.next().unwrap();

            let has_first = password.chars().nth(first - 1).unwrap() == character;
            let has_second = password.chars().nth(second - 1).unwrap() == character;
            return has_first ^ has_second
        })
        .count();

    println!("{}", result);
}

