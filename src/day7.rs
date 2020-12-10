use super::*;

pub fn run(input: &Vec<String>) {
    first(input);
    second(input);
}

fn first(input: &Vec<String>) {
    let rules = parse_rules(input);
    // Go through and simplify all final mappings
    // Ignoring counts
    let result = rules.keys().to_owned()
        .filter(|bag| does_bag_contain_shiny(bag, &rules))
        .count();
    println!("{}", result);
}

fn second(input: &Vec<String>) {
    let rules = parse_rules(input);
    // Go expand the shiny bag until all bags terminate
    let result = expand_until_terminate("shiny gold", &rules);
    println!("{}", result);
}

fn expand_until_terminate(bag_name: &str, rules: &HashMap<String, Vec<Mapping>>) -> i32 {
    let mappings = rules.get(bag_name).unwrap();
    if mappings.is_empty() {
        return 0;
    }
    let mut count = 0;
    for map in mappings {
        count += map.amount + map.amount * expand_until_terminate(&*map.bag_name, &rules);
    }
    count
}

fn does_bag_contain_shiny(bag_name: &str, rules: &HashMap<String, Vec<Mapping>>) -> bool {
    let mappings = rules.get(bag_name).unwrap();
    if mappings.is_empty() {
        return false;
    } else if mappings.iter().any(|i| i.bag_name == "shiny gold") {
        return true;
    }
    return mappings.iter()
        .any(|mapping| does_bag_contain_shiny(&*mapping.bag_name, rules))
}

// Shiny bag == 0
fn parse_rules(input: &Vec<String>) -> HashMap<String, Vec<Mapping>> {
    let mut rules = HashMap::new();
    for line in input {
        let (name, mappings) = extract_from_line(line);
        assert!(!rules.contains_key(&name));
        rules.insert(name, mappings);
    }
    rules
}

fn extract_from_line(line: &String) -> (String, Vec<Mapping>) {
    let (name, containers) = line.split_once(" bags contain ").unwrap();
    let mut mappings = Vec::new();
    for item in containers.split(",") {
        if item == "no other bags." {
            // lol
            break;
        }
        let mut split = item.trim().split(" ");
        let count = split.next().unwrap().parse::<i32>().unwrap();
        let bag_name = split.next().unwrap().to_owned() + " " +  split.next().unwrap();
        mappings.push(Mapping::new(bag_name.to_owned(), count));
    }
    (name.to_owned(), mappings)
}

#[derive(Debug, Eq, PartialEq)]
struct Mapping {
    pub bag_name: String,
    pub amount: i32,
}

impl Mapping {
    pub fn new(bag_name: String, amount: i32) -> Self {
        Self {
            bag_name,
            amount,
        }
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