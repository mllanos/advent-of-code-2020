use std::fs;

pub fn run() {
    let contents = fs::read_to_string("src/day_1/report_repair.txt")
        .expect("Something went wrong reading the file");
    let values: Vec<&str> = contents.split("\n").collect();
    let mut count = 0;
    for (i, x) in values.iter().enumerate() {
        let a: i32 = x.parse().unwrap();
        for y in values.clone().drain(i+1..values.len()) {
            count = count + 1;
            let b: i32 = y.parse().unwrap();
            if a + b == 2020 {
                print!("{} + {} = 2020\n", a, b);
                print!("{} * {} = {}\n", a, b, a * b);
            }
        }
    }
    print!("Amount of iterations: {}\n", count);
}
