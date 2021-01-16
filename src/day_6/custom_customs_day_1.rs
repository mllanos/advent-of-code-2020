use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::str;

#[test]
fn validate_6_1() {
    assert_eq!(algorithm("src/day_6/input_test.txt"), 11);
}

fn algorithm(file_location: &str) -> usize {
    let content = fs::read_to_string(file_location).unwrap();
    let delimiter = Regex::new("\n\\s+\n").unwrap();
    let forms: Vec<&str> = delimiter.split(&content).collect();
    let mut result = 0;
    for group in forms {
        let chars: Vec<char> = group
            .chars()
            .filter(|c| c.is_alphabetic())
            .into_iter()
            .unique()
            .collect();
        result += chars.len();
    }
    result
}

pub fn run() {
    println!("The result is: {}.", algorithm("src/day_6/input.txt"));
}
