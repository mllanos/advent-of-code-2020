use std::fs;
use std::str;

#[test]
fn validate() {
    assert_eq!(algorithm("src/day_8/input_test.txt"), 5);
}

fn algorithm(file_location: &str) -> i32 {
    let content = fs::read_to_string(file_location).unwrap();
    let mut operations: Vec<(&str, i32)> = vec![];
    let mut accumulator = 0;
    let mut index: i32 = 0;
    let mut visited_indexes: Vec<i32> = vec![];

    for line in content.lines() {
        let splitted: Vec<&str> = line.split(" ").collect();
        operations.push((splitted[0], splitted[1].parse().unwrap()));
    }

    loop {
        let (opcode, operand) = operations[index as usize];
        visited_indexes.push(index);
        match opcode {
            "nop" => index = index + 1,
            "acc" => {
                index = index + 1;
                accumulator = accumulator + operand;
            }
            "jmp" => index = index + operand,
            _ => panic!("Invalid opcode."),
        }
        if visited_indexes.contains(&index) {
            break;
        }
    }

    accumulator
}

pub fn run() {
    println!(
        "The value in the accumulator is {}.",
        algorithm("src/day_8/input.txt")
    );
}
