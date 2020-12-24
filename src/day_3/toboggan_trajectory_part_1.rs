use std::fs;

pub fn run() {
    let contents = fs::read_to_string("src/day_3/input.txt")
        .expect("Something went wrong reading the file");
    let values: Vec<&str> = contents.split("\n").collect();
    let max_cols = values.first().unwrap().len() - 1;
    let mut count = 0;
    let mut curr_col = 0;

    for (i, row) in values.iter().enumerate() {
        let target = row.chars().nth(curr_col).unwrap();
        if target == '#' && i != 0 {
            count = count + 1;
        }
        curr_col = (curr_col + 3) % max_cols;
    }
    print!("Right 3, down 1: we hit {} trees.\n", count);
}
