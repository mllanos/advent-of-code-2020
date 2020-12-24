use std::fs;

pub fn run() {
    let contents = fs::read_to_string("src/day_3/input.txt")
        .expect("Something went wrong reading the file");
    let values: Vec<&str> = contents.split("\n").collect();
    let max_cols = values.first().unwrap().len() - 1;
    let max_rows = values.len() - 1;
    let mut total: u64 = 1;
    let slope_strategies = [(1,1), (3,1), (5,1), (7,1), (1,2)];

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

        print!("Right {}, down {}: we hit {} trees.\n", right, down, count);
    }
    print!("Multiplying all the values give us {}.\n", total);
}
