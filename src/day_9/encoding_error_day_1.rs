use std::fs;
use std::str;

#[test]
fn validate_9_1() {
    assert_eq!(algorithm("src/day_9/input_test.txt", 5), 127);
}

fn algorithm(file_location: &str, preamble: usize) -> i64 {
    let content = fs::read_to_string(file_location).unwrap();
    let mut xmas_codes: Vec<i64> = vec![];
    let mut preamble_codes: Vec<i64> = vec![];
    let mut curr_code: i64 = -1;

    for (index, line) in content.lines().enumerate() {
        let xmas_code = line.parse().unwrap();
        if index < preamble {
            preamble_codes.push(xmas_code);
        } else {
            xmas_codes.push(xmas_code);
        }
    }

    for xmas_code in xmas_codes {
        let mut valid = false;
        curr_code = xmas_code;
        for preamble_code in preamble_codes.clone() {
            let remainder = xmas_code - preamble_code;
            if preamble_codes.contains(&remainder) {
                valid = true;
                break;
            }
        }

        if !valid {
            break;
        }

        preamble_codes.remove(0);
        preamble_codes.push(xmas_code);
    }

    curr_code
}

pub fn run() {
    println!(
        "The first number that does not have the property is {}.",
        algorithm("src/day_9/input.txt", 25)
    );
}
