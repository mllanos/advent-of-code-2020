use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::str;

fn get_passport_hashmap(passport: &str) -> HashMap<&str, &str> {
    let mut passport_hashmap = HashMap::new();
    let re = Regex::new("\n| ").unwrap();
    let substrings: Vec<&str> = re.split(passport).collect();
    for substring in substrings.iter() {
        let helper: Vec<&str> = substring.split(":").collect();
        passport_hashmap.insert(helper[0], helper[1].trim());
    }
    passport_hashmap
}

pub fn run() {
    let content = fs::read_to_string("src/day_4/input.txt").unwrap();
    let delimiter = Regex::new("\n\\s+\n").unwrap();
    let passports: Vec<&str> = delimiter.split(&content).collect();
    let keys = [
        ("byr", Regex::new("^19[2-9][0-9]|200[0-2]$").unwrap()),
        ("iyr", Regex::new("^201[0-9]|2020$").unwrap()),
        ("eyr", Regex::new("^202[0-9]|2030$").unwrap()),
        (
            "hgt",
            Regex::new("^1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in$").unwrap(),
        ),
        ("hcl", Regex::new("^#[0-9a-f]{6}$").unwrap()),
        ("ecl", Regex::new("^amb|blu|brn|gry|grn|hzl|oth$").unwrap()),
        ("pid", Regex::new("^[0-9]{9}$").unwrap()),
    ];
    let mut matches = 0;
    for passport in passports.iter() {
        let passport_hashmap = get_passport_hashmap(passport);
        if keys.iter().all(|(key, re)| {
            passport_hashmap.contains_key(key) && re.is_match(passport_hashmap.get(key).unwrap())
        }) {
            matches = matches + 1;
        }
    }
    println!(
        "Out of {} passports, {} are valid.",
        passports.len(),
        matches
    );
}
