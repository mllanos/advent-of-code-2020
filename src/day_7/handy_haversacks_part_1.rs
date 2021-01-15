use std::fs;
use std::str;
use regex::Regex;
use std::collections::HashMap;

#[test]
fn validate() {
    assert_eq!(algorithm("src/day_7/input_test.txt"), 4);
}

fn algorithm(file_location: &str) -> usize {
    let rule_delimiter = Regex::new("s contain ([0-9]+ )?|s?, [0-9]+ ").unwrap();
    let content = fs::read_to_string(file_location).unwrap();
    let mut matches = 0;
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in content.lines() {
        let mut bags: Vec<&str> = rule_delimiter.split(&line.trim_end_matches(".").trim_end_matches("s")).collect();
        let first = bags.remove(0);
        rules.insert(first, bags);
    }

    rules.insert("no other bag", vec![]);

    for (_, mut possible_contained_bags) in rules.clone() {
        loop {
            if possible_contained_bags.contains(&"shiny gold bag") {
                matches = matches + 1;
                break;
            }
            possible_contained_bags = possible_contained_bags
                .into_iter()
                .flat_map(|bag| rules.get(bag).unwrap().to_vec())
                .collect::<Vec<_>>();
            if possible_contained_bags.is_empty() {
                break;
            }
        }
    }

  
    matches
}

pub fn run() {
    println!("The number of bag colors that can contain one shiny gold bag is {}.", algorithm("src/day_7/input.txt"));
}