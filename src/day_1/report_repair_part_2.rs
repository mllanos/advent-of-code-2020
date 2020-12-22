use std::fs;

pub fn run() {
    let contents = fs::read_to_string("src/day_1/input.txt")
        .expect("Something went wrong reading the file");
    let values: Vec<&str> = contents.split("\n").collect();
    let mut count = 0;
    for (i, x) in values.iter().enumerate() {
        let a: i32 = x.parse().unwrap();
        let mut count_inner = 0;
        for y in values.clone().drain(i+1..values.len()) {
            let b: i32 = y.parse().unwrap();
            count_inner = count_inner + 1;
            for z in values.clone().drain(i + count_inner + 1..values.len()) {
                let c: i32 = z.parse().unwrap();
                count = count + 1;
                if a + b + c == 2020 {
                    print!("{} + {} + {} = 2020\n", a, b, c);
                    print!("{} * {} * {} = {}\n", a, b, c, a * b * c);
                }
            }
        }
    }
    print!("Amount of iterations: {}\n", count);
}
