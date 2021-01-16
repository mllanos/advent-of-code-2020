use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::str;

#[test]
fn validate_6_2() {
    assert_eq!(algorithm("src/day_6/input_test.txt"), 6);
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
        let lines_count = group.lines().count();
        for chr in chars {
            if group.matches(chr).count() == lines_count {
                result += 1;
            }
        }
    }
    result
}

pub fn run() {
    println!("The result is: {}.", algorithm("src/day_6/input.txt"));
}
