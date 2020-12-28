use std::fs;
use regex::Regex;

fn parse_data(s: &str) -> Vec<&str> {
    let re = Regex::new(":? |-").unwrap();
    re.split(s).collect()
}

pub fn run() {
    let contents = fs::read_to_string("src/day_2/input.txt")
        .expect("Something went wrong reading the file");
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
    print!("Out of {} passwords, {} are valid.\n", values.len(), count);
}
