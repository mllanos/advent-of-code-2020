use std::fs;
use std::str;

#[test]
fn validate_9_2() {
    assert_eq!(algorithm("src/day_9/input_test.txt", 5), 62);
}

fn algorithm(file_location: &str, preamble: usize) -> i64 {
    let content = fs::read_to_string(file_location).unwrap();
    let mut xmas_codes: Vec<i64> = vec![];
    let mut preamble_codes: Vec<i64> = vec![];
    let mut curr_code: i64 = -1;
    let mut contiguous_range: Vec<i64> = vec![];
    let mut total_range: Vec<i64> = vec![];

    for (index, line) in content.lines().enumerate() {
        let xmas_code = line.parse().unwrap();
        if index < preamble {
            preamble_codes.push(xmas_code);
        } else {
            xmas_codes.push(xmas_code);
        }
        total_range.push(xmas_code);
    }

    for xmas_code in xmas_codes.clone() {
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

    for (i, xmas_code) in total_range.clone().into_iter().enumerate() {
        let mut range_valid = false;
        contiguous_range = vec![xmas_code];
        for other_xmas_code in total_range.clone().drain(i + 1..xmas_codes.len()) {
            contiguous_range.push(other_xmas_code);
            let range_sum: i64 = contiguous_range.clone().into_iter().sum();
            if curr_code <= range_sum {
                if curr_code == range_sum {
                    range_valid = true;
                }
                break;
            }
        }

        if range_valid {
            break;
        }
    }

    let min = contiguous_range.iter().min_by(|x, y| x.cmp(y)).unwrap();
    let max = contiguous_range.iter().max_by(|x, y| x.cmp(y)).unwrap();

    min + max
}

pub fn run() {
    println!(
        "The encryption weakness in the XMAS-encrypted list is {}.",
        algorithm("src/day_9/input.txt", 25)
    );
}
