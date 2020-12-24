use std::fs;
use std::str;
use regex::Regex;

pub fn run() {
    let content = fs::read_to_string("src/day_4/input.txt").unwrap();
    let delimiter = Regex::new("\n\\s+\n").unwrap();
    let passports: Vec<&str> = delimiter.split(&content).collect();
    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut matches = 0;
    for passport in passports.iter() {
        if keys.iter().all(|key| passport.contains(key)) {
            matches = matches + 1;
        }
    }
    print!("Out of {} passports, {} are valid.\n", passports.len(), matches);
}
