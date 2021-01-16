use std::fs;

#[test]
fn validate() {
    assert_eq!(algorithm("src/day_3/input_test.txt", false), 336);
}

fn algorithm(file_location: &str, print_results: bool) -> u64 {
    let contents = fs::read_to_string(file_location).unwrap();
    let values: Vec<&str> = contents.lines().collect();
    let max_cols = values.first().unwrap().len();
    let max_rows = values.len() - 1;
    let mut total: u64 = 1;
    let slope_strategies = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (right, down) in slope_strategies.iter() {
        let mut curr_row = 0;
        let mut curr_col = 0;
        let mut first_iter = true;
        let mut done = false;
        let mut count = 0;

        while !done {
            let row = values[curr_row];
            let target = row.chars().nth(curr_col).unwrap();
            if target == '#' && !first_iter {
                count = count + 1;
            }
            curr_row = curr_row + down;
            curr_col = (curr_col + right) % max_cols;
            if curr_row > max_rows {
                done = true;
            }
            first_iter = false;
        }

        total = total * count;

        if print_results {
            println!("Right {}, down {}: we hit {} trees.", right, down, count);
        }
    }
    total
}

pub fn run() {
    println!(
        "Multiplying all the values gives us {}.",
        algorithm("src/day_3/input.txt", true)
    );
}
