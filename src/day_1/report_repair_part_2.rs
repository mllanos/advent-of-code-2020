use std::fs;

#[test]
fn validate_1_2() {
    assert_eq!(algorithm("src/day_1/input_test.txt", false).0, 241861950);
}

fn algorithm(file_location: &str, print_results: bool) -> (i32, usize) {
    let contents = fs::read_to_string(file_location).unwrap();
    let values: Vec<&str> = contents.lines().collect();
    let mut count = 0;
    let mut result = 0;
    for (i, x) in values.iter().enumerate() {
        let a: i32 = x.parse().unwrap();
        let mut count_inner = 0;
        for y in values.clone().drain(i + 1..values.len()) {
            let b: i32 = y.parse().unwrap();
            count_inner = count_inner + 1;
            for z in values.clone().drain(i + count_inner + 1..values.len()) {
                let c: i32 = z.parse().unwrap();
                count = count + 1;
                if a + b + c == 2020 {
                    result = a * b * c;
                    if print_results {
                        println!("{} + {} + {} = 2020", a, b, c);
                        println!("{} * {} * {} = {}", a, b, c, result);
                    }
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
