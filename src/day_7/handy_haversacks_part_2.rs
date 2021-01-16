use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::str;

#[test]
fn validate_7_2() {
    assert_eq!(algorithm("src/day_7/input_test.txt"), 32);
    assert_eq!(algorithm("src/day_7/input_test2.txt"), 126);
}

fn algorithm(file_location: &str) -> i32 {
    let rule_delimiter = Regex::new("s contain |s?, ").unwrap();
    let content = fs::read_to_string(file_location).unwrap();
    let mut matches = 0;
    let mut rules: HashMap<&str, Vec<(i32, &str)>> = HashMap::new();
    for line in content.lines() {
        let mut bags: Vec<&str> = rule_delimiter
            .split(&line.trim_end_matches(".").trim_end_matches("s"))
            .collect();
        let first = bags.remove(0);
        rules.insert(
            first,
            bags.into_iter()
                .map(|x| {
                    if x == "no other bag" {
                        return (1, x);
                    }
                    let mut splitter = x.splitn(2, ' ');
                    let first = splitter.next().unwrap();
                    let second = splitter.next().unwrap();
                    (first.parse().unwrap(), second)
                })
                .collect(),
        );
    }

    let mut bag_contains = rules.get(&"shiny gold bag").unwrap().to_vec();

    loop {
        let mut tmp: Vec<(i32, &str)> = vec![];
        for (amount, bag) in bag_contains {
            let contained_bags = rules.get(bag).unwrap().to_vec();
            matches = matches + amount;
            for (other_amount, other_bag) in contained_bags {
                if other_bag != "no other bag" {
                    tmp.push((other_amount * amount, other_bag))
                }
            }
        }
        bag_contains = tmp;
        if bag_contains.is_empty() {
            break;
        }
    }
    matches
}

pub fn run() {
    println!(
        "The number of individual bags required inside your shiny gold bag is {}.",
        algorithm("src/day_7/input.txt")
    );
}
