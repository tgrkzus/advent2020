use super::*;

pub fn run(input: &Vec<String>) {
    first(input);
    second(input);
}

fn first(input: &Vec<String>) {
    let mut nums: Vec<i64> = input.iter()
        .map(|line| line.parse().unwrap())
        .collect();
    nums.push(0);
    nums.push(nums.iter().max().unwrap() + 3);
    nums.sort();
    let mut ones = 0;
    let mut threes = 0;
    for x in 0..(nums.len() - 1) {
        let diff = (nums[x + 1] - nums[x]).abs();
        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }
    }
    println!("{}", ones * threes);
}

fn second(input: &Vec<String>) {
    let mut nums: Vec<i64> = input.iter()
        .map(|line| line.parse().unwrap())
        .collect();
    nums.push(0);
    nums.push(nums.iter().max().unwrap() + 3);
    nums.sort();
    let amount = walk(&nums, &vec![0], &mut HashMap::new());
    println!("{}", amount);
}

fn walk(source: &Vec<i64>, choices: &Vec<usize>, mut cache: &mut HashMap<usize, i64>) -> i64 {
    let last = choices.last().unwrap();
    let allowed = {
        let mut allow = Vec::new();
        let value = source[*last];
        for x in (last + 1)..source.len() {
            if (source[x] - value).abs() > 3 {
                break;
            }
            allow.push(x);
        }
        allow
    };
    if allowed.is_empty() {
        // dead end
        return 0;
    }
    if *allowed.first().unwrap() == source.len() - 1 {
        return 1;
    }
    let mut result = 0;
    for choice in allowed {
        if !cache.contains_key(&choice) {
            let val = walk(&source, &add_to_and_clone(&choices, choice), &mut cache);
            cache.insert(choice, val);
        }
        result += cache.get(&choice).unwrap();
    }
    return result;
}

fn add_to_and_clone(choices: &Vec<usize>, val: usize) -> Vec<usize> {
    let mut new = choices.clone();
    new.push(val);
    new
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answers() {
    }
}
