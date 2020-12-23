use std::fs;
use regex::Regex;

fn parse_data(s: &str) -> Vec<&str> {
    let re = Regex::new(":? |-").unwrap();
    re.split(s).collect()
}

pub fn run() {
    let contents = fs::read_to_string("src/day_2/input.txt")
        .expect("Something went wrong reading the file");
    let values: Vec<Vec<&str>> = contents.split("\n").map(parse_data).collect();
    let mut count = 0;
    for item in values.iter() {
        let (min, max, letter, password) = (item[0], item[1], item[2].chars().next().unwrap(), item[3]);
        let min_times: usize = min.parse().unwrap();
        let max_times: usize = max.parse().unwrap();
        let times: usize = password.chars().filter(|c| c == &letter).collect::<Vec<_>>().len();
        if min_times <= times && times <= max_times {
            count = count + 1;
        }
    }
    print!("Out of {} passwords, {} are valid.\n", values.len(), count);
}
