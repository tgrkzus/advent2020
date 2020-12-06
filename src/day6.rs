use super::*;
use itertools::__std_iter::FromIterator;

pub fn run(input: &Vec<String>) {
    first(input);
    second(input);
}

fn first(input: &Vec<String>) {
    let mut groups: Vec<HashSet<char>> = Vec::new();
    let mut current: HashSet<char> = HashSet::new();
    for line in input {
        if line.is_empty() {
            let old = current;
            current = HashSet::new();
            groups.push(old);
            continue;
        }
        current = current.union(&HashSet::from_iter(line.chars())).map(|c| *c).collect();
    }
    groups.push(current);
    let result: usize = groups.iter()
        .map(|group| group.len())
        .sum();
    println!("{}", result);
}

fn second(input: &Vec<String>) {
    let mut groups: Vec<HashSet<char>> = Vec::new();
    let mut current: HashSet<char> = HashSet::new();
    let mut first = true;
    for line in input {
        if line.is_empty() {
            let old = current;
            current = HashSet::new();
            groups.push(old);
            first = true;
            continue;
        }
        let this_person = HashSet::from_iter(line.chars());
        if first {
            current = this_person;
            first = false;
        } else {
            current = current.intersection(&this_person)
                .map(|c| *c)
                .collect();
        }
    }
    groups.push(current);
    let result: usize = groups.iter()
        .map(|group| group.len())
        .sum();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answers() {
    }
}