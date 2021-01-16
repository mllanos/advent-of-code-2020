use std::fs;

#[test]
fn validate_3_1() {
    assert_eq!(algorithm("src/day_3/input_test.txt"), 7);
}

fn algorithm(file_location: &str) -> usize {
    let contents = fs::read_to_string(file_location).unwrap();
    let values: Vec<&str> = contents.lines().collect();
    let max_cols = values.first().unwrap().len();
    let mut count = 0;
    let mut curr_col = 0;

    for (i, row) in values.iter().enumerate() {
        let target = row.chars().nth(curr_col).unwrap();
        if target == '#' && i != 0 {
            count = count + 1;
        }
        curr_col = (curr_col + 3) % max_cols;
    }
    count
}

pub fn run() {
    println!(
        "Right 3, down 1: we hit {} trees.",
        algorithm("src/day_3/input.txt")
    );
}
