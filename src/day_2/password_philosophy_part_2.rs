use std::fs;
use regex::Regex;

#[test]
fn validate() {
    assert_eq!(algorithm("src/day_2/input_test.txt"), (3, 1));
}

fn parse_data(s: &str) -> Vec<&str> {
    let re = Regex::new(":? |-").unwrap();
    re.split(s).collect()
}

fn algorithm(file_location: &str) -> (usize, usize) {
    let contents = fs::read_to_string(file_location).unwrap();
    let values: Vec<Vec<&str>> = contents.lines().map(parse_data).collect();
    let mut count = 0;

    for item in values.iter() {
        let (min, max, letter, password) = (item[0], item[1], item[2].chars().next().unwrap(), item[3]);
        let position_1: usize = min.parse().unwrap();
        let position_2: usize = max.parse().unwrap();
        if password.len() + 1 > position_2 {
            let char_pos_1 = password.chars().nth(position_1 - 1).unwrap();
            let char_pos_2 = password.chars().nth(position_2 - 1).unwrap();
            
            if (char_pos_1 == letter && char_pos_2 != letter) || (char_pos_1 != letter && char_pos_2 == letter) {
                count = count + 1;
            }
        }
    }
    
    (values.len(), count)
}

pub fn run() {
    let (passwords, valid) = algorithm("src/day_2/input.txt");
    println!("Out of {} passwords, {} are valid.", passwords, valid);
}