use std::fs;
use std::str;

#[test]
fn validate_7_2() {
    assert_eq!(algorithm("src/day_8/input_test.txt"), 8);
}

fn algorithm(file_location: &str) -> i32 {
    let content = fs::read_to_string(file_location).unwrap();
    let mut operations: Vec<(&str, i32)> = vec![];
    let mut accumulator: i32;
    let mut index: i32;
    let mut attempted_override_indexes: Vec<i32> = vec![];

    for line in content.lines() {
        let splitted: Vec<&str> = line.split(" ").collect();
        operations.push((splitted[0], splitted[1].parse().unwrap()));
    }

    loop {
        let mut visited_indexes: Vec<i32> = vec![];
        let mut overriden = false;
        let mut terminated = false;
        accumulator = 0;
        index = 0;
        loop {
            let (mut opcode, operand) = operations[index as usize];
            visited_indexes.push(index);

            if vec!["nop", "jmp"].contains(&opcode)
                && !attempted_override_indexes.contains(&index)
                && !overriden
            {
                opcode = match opcode {
                    "nop" => "jmp",
                    "jmp" => "nop",
                    _ => panic!("Invalid opcode."),
                };
                attempted_override_indexes.push(index);
                overriden = true;
            }

            match opcode {
                "nop" => index = index + 1,
                "acc" => {
                    index = index + 1;
                    accumulator = accumulator + operand;
                }
                "jmp" => index = index + operand,
                _ => panic!("Invalid opcode."),
            }

            if index >= operations.len() as i32 {
                terminated = true;
            }
            if visited_indexes.contains(&index) || terminated {
                break;
            }
        }
        if terminated {
            break;
        }
    }

    accumulator
}

pub fn run() {
    println!(
        "The value of the accumulaotr after the program terminates is {}.",
        algorithm("src/day_8/input.txt")
    );
}
