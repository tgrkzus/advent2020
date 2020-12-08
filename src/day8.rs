use super::*;
use itertools::__std_iter::FromIterator;

pub fn run(input: &Vec<String>) {
    first(input);
    second(input);
}

fn first(input: &Vec<String>) {
    let result;
    let program = generate_program(input);
    let mut machine = Machine::new();
    let mut has_run = HashSet::new();
    loop {
        let to_run = machine.pointer;
        if has_run.contains(&to_run) {
            result = machine.accumulator;
            break;
        }
        machine.step(&program);
        has_run.insert(to_run);
    }

    println!("{}", result);
}

fn second(input: &Vec<String>) {
    let original_program = generate_program(input);
    let mut program = original_program.clone();
    let mut result;
    loop {
        if let Some(acc) = try_program(&program) {
            result = acc;
            break;
        } else {
            // Randomly flip a operation from the original program.
            program = original_program.clone();
            let mut rng = rand::thread_rng();
            loop {
                let rand = rng.gen_range(0, original_program.len());
                match original_program.get(rand).unwrap() {
                    Operation::Jmp(arg) => {
                        std::mem::replace(&mut program[rand], Operation::Nop(*arg));
                        break;
                    },
                    Operation::Nop(arg) => {
                        std::mem::replace(&mut program[rand], Operation::Jmp(*arg));
                        break;
                    },
                    Operation::Acc(_) => continue
                }
            }
        }
    }
    println!("{}", result);
}

fn try_program(program: &Vec<Operation>) -> Option<i32> {
    let mut machine = Machine::new();
    let mut has_run = HashSet::new();
    loop {
        let to_run = machine.pointer;
        if has_run.contains(&to_run) {
            // If we try another operation we know it's looping
            return None
        }
        if !machine.step(&program) {
            // Return acc if program terminates cleanly
            return Some(machine.accumulator)
        }
        has_run.insert(to_run);
    }


}

fn generate_program(input: &Vec<String>) -> Vec<Operation> {
    let mut program = Vec::new();
    for line in input {
        let (operation_string, argument_string) = line.split_once(" ").unwrap();
        let argument = argument_string.parse().unwrap();
        let oper = match operation_string {
            "acc" => Operation::Acc(argument),
            "jmp" => Operation::Jmp(argument),
            "nop" => Operation::Nop(argument),
            _ => panic!("invalid operation {}", operation_string),
        };
        program.push(oper);
    }
    program
}

#[derive(Clone)]
enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

struct Machine {
    pub accumulator: i32,
    pub pointer: i32,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            pointer: 0,
        }
    }

    pub fn step(&mut self, program: &Vec<Operation>) -> bool {
        if self.pointer == program.len() as i32 {
            // Program terminates
            return false;
        }
        let operation: &Operation = program.get(self.pointer as usize).expect("Could not find operation");
        match operation {
            Operation::Acc(arg) => {
                self.accumulator += arg;
                self.pointer += 1;
            },
            Operation::Jmp(arg) => {
                self.pointer += arg;
            },
            Operation::Nop(arg) => {
                self.pointer += 1;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answers() {
        assert_eq!(
            extract_from_line(&"light red bags contain 1 bright white bag, 2 muted yellow bags.".to_owned()),
            ("light red".to_owned(), vec![Mapping::new("bright white".to_owned(), 1), Mapping::new("muted yellow".to_owned(), 2)])
        );
        assert_eq!(
            extract_from_line(&"bright purple bags contain no other bags.".to_owned()),
            ("bright purple".to_owned(), vec![])
        );
    }
}
