use super::*;

pub fn run(input: &Vec<String>) {
    first(input);
    second(input);
}

fn first(input: &Vec<String>) {
    let result = input.iter()
        .map(|v| v.parse::<u32>().unwrap())
        .combinations(2)
        .filter(|comb| comb[0] + comb[1] == 2020)
        .last()
        .map(|comb| comb[0] * comb[1])
        .unwrap();
    println!("{}", result);
}

fn second(input: &Vec<String>) {
    let result = input.iter()
        .map(|v| v.parse::<u32>().unwrap())
        .combinations(3)
        .filter(|comb| comb[0] + comb[1] + comb[2] == 2020)
        .last()
        .map(|comb| comb[0] * comb[1] * comb[2])
        .unwrap();
    println!("{}", result);
}
