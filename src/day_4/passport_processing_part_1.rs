use std::fs;
use std::str;
use regex::Regex;

#[test]
fn validate() {
    assert_eq!(algorithm("src/day_4/input_test.txt"), (4, 2));
}

fn algorithm(file_location: &str) -> (usize, usize) {
    let content = fs::read_to_string(file_location).unwrap();
    let delimiter = Regex::new("\n\\s+\n").unwrap();
    let passports: Vec<&str> = delimiter.split(&content).collect();
    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut matches = 0;
    for passport in passports.iter() {
        if keys.iter().all(|key| passport.contains(key)) {
            matches = matches + 1;
        }
    }
    (passports.len(), matches)
}

pub fn run() {
    let (passports, valid) = algorithm("src/day_4/input.txt");
    println!("Out of {} passports, {} are valid.", passports, valid);
}
