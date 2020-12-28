use std::fs;
use std::str;

pub fn run() {
    // F,L -> 0, B,R -> 1
    let content: std::string::String = fs::read_to_string("src/day_5/input.txt").unwrap()
        .chars()
        .map(|x| match x { 
            'F' => '0',
            'L' => '0',
            'B' => '1',
            'R' => '1',
            _ => x
        }).collect();
    let passes: Vec<Vec<isize>> = content.split("\n")
        .map(|x| x.trim().as_bytes().chunks(7).map(str::from_utf8).map(|a| isize::from_str_radix(a.unwrap(), 2)).collect::<Result<Vec<isize>, _>>().unwrap())
        .collect::<Vec<Vec<isize>>>();
    let mut max_seat_id = 0;
    for pass in passes.iter() {
        let seat_id = pass[0] * 8 + pass[1];
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }
    print!("The highest seat ID on the boarding pass is {}.\n", max_seat_id);
}
