use super::*;

pub fn run(input: &Vec<String>) {
    first(input);
    second(input);
}

fn first(input: &Vec<String>) {
    let nums: Vec<i64> = input.iter()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut result = None;
    let jump= 25;
    for index in jump..nums.len() {
        let sums = make_sums(&nums, index - jump, jump);
        let to_check = nums[index];
        if !sums.contains(&to_check) {
            result = Some(to_check);
            break;
        }
    }
    println!("{:?}", result);
}

fn second(input: &Vec<String>) {
    let nums: Vec<i64> = input.iter()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut result = None;
    let jump= 25;
    for index in jump..nums.len() {
        let sums = make_sums(&nums, index - jump, jump);
        let to_check = nums[index];
        if !sums.contains(&to_check) {
            result = Some(to_check);
            break;
        }
    }
    if let Some(to_find) = result {
        let (lowest, highest) = find_cont_set_summing(&nums, to_find).unwrap();
        let result = lowest + highest;
        println!("{:?}", result);
    } else {
        panic!("hwat");
    }
}

fn find_cont_set_summing(source: &Vec<i64>, to_find: i64) -> Option<(i64, i64)>{
    // At least 2 numbers so (len - 2)
    for len in 2..(source.len() - 2) {
        for start in 0..(source.len() - len) {
            let set = &source[start..(start + len)];
            let sum: i64 = set.iter().sum();
            if sum == to_find {
                return Some((*set.iter().min().unwrap(), *set.iter().max().unwrap()));
            }
        }
    }
    return None
}

fn make_sums(source: &Vec<i64>, start: usize, amount: usize) -> HashSet<i64> {
    source[start..(start + amount)]
        .iter()
        .combinations(2)
        .map(|comb| comb[0] + comb[1])
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answers() {
    }
}
