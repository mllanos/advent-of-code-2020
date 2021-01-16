use std::fs;

#[test]
fn validate_1_1() {
    assert_eq!(algorithm("src/day_1/input_test.txt", false).0, 514579);
}

fn algorithm(file_location: &str, print_results: bool) -> (i32, usize) {
    let contents = fs::read_to_string(file_location).unwrap();
    let values: Vec<&str> = contents.lines().collect();
    let mut count = 0;
    let mut result = 0;
    for (i, x) in values.iter().enumerate() {
        let a: i32 = x.parse().unwrap();
        for y in values.clone().drain(i + 1..values.len()) {
            count = count + 1;
            let b: i32 = y.parse().unwrap();
            if a + b == 2020 {
                result = a * b;
                if print_results {
                    println!("{} + {} = 2020", a, b);
                    println!("{} * {} = {}", a, b, result);
                }
            }
        }
    }

    (result, count)
}

pub fn run() {
    let (result, iterations) = algorithm("src/day_1/input.txt", true);
    println!("Result: {}. Amount of iterations: {}.", result, iterations);
}
