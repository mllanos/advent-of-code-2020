use std::fs;
use std::str;

#[test]
fn validate_10_1() {
    assert_eq!(algorithm("src/day_10/input_test.txt"), 35);
    assert_eq!(algorithm("src/day_10/input_test2.txt"), 220);
}

fn algorithm(file_location: &str) -> u32 {
    let content = fs::read_to_string(file_location).unwrap();
    let mut joltages: Vec<u32> = vec![];
    let mut max_joltage = 0;
    let mut previous_joltage = 0;
    let mut _1_jolt_differences = 0;
    let mut _3_jolt_differences = 0;

    for line in content.lines() {
        let joltage = line.parse().unwrap();
        if joltage > max_joltage {
            max_joltage = joltage;
        }
        joltages.push(joltage);
    }

    joltages.sort();
    joltages.push(max_joltage + 3);

    for joltage in joltages {
        if previous_joltage + 1 == joltage {
            _1_jolt_differences = _1_jolt_differences + 1;
        }
        if previous_joltage + 3 == joltage {
            _3_jolt_differences = _3_jolt_differences + 1;
        }
        previous_joltage = joltage;
    }

    _1_jolt_differences * _3_jolt_differences
}

pub fn run() {
    println!(
        "The number of 1-jolt differences multiplied by the number of 3-jolt differences is {}.",
        algorithm("src/day_10/input.txt")
    );
}
