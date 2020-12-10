use super::*;

pub fn run(input: &Vec<String>) {
    first(input);
    second(input);
}

fn first(input: &Vec<String>) {
    let result = input.iter()
        .map(|line| to_answer(line))
        .max_by(|a, b| a.2.cmp(&b.2))
        .unwrap();
    println!("{}", result.2);
}

fn second(input: &Vec<String>) {
    let results: HashSet<i32> = input.iter()
        .map(|line| to_answer(line).2)
        .collect();
    let result = {
        let val;
        let mut i = 1;
        loop {
            if results.contains(&i) && results.contains (&(i + 2)) && !results.contains(&(i + 1)) {
                val = i + 1;
                break;
            }
            i += 1;
        }
        val
    };
    println!("{}", result);
}

fn to_answer(input: &str) -> (i32, i32, i32) {
    let row_dirs = input.chars().take(7)
        .map(|c| if c == 'F' { Direction::Lower } else { Direction::Upper })
        .collect();
    let col_dirs = input.chars().skip(7).take(3)
        .map(|c| if c == 'L' { Direction::Lower } else { Direction::Upper })
        .collect();
    let row = process_steps(row_dirs);
    let col = process_steps(col_dirs);
    (row, col, row * 8 + col)
}

fn process_steps(dirs: Vec<Direction>) -> i32 {
    let mut lower = 0;
    let mut upper = 2_i32.pow(dirs.len() as u32) - 1;
    for dir in dirs {
        match dir {
            Direction::Upper => lower += (diff(lower, upper) as f32 / 2.0).ceil() as i32,
            Direction::Lower => upper -= (diff(lower, upper) as f32 / 2.0).ceil() as i32,
        }
    }
    assert_eq!(lower, upper);
    lower
}

fn diff(a: i32, b: i32) -> i32 {
    (a - b).abs()
}

enum Direction {
    Upper,
    Lower,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answers() {
        assert_eq!(to_answer("FBFBBFFRLR"), (44, 5, 357));
        assert_eq!(to_answer("BFFFBBFRRR"), (70, 7, 567));
        assert_eq!(to_answer("FFFBBBFRRR"), (14, 7, 119));
        assert_eq!(to_answer("BBFFBBFRLL"), (102, 4, 820));
    }
}