use std::fs;
use std::str;
use itertools::Itertools;

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
    let seat_ids: Vec<isize> = content.split("\n")
        .map(|x| x.trim().as_bytes().chunks(7).map(str::from_utf8).map(|a| isize::from_str_radix(a.unwrap(), 2)).collect::<Result<Vec<isize>, _>>().unwrap())
        .map(|x| x[0] * 8 + x[1])
        .collect::<Vec<isize>>()
        .into_iter().sorted().collect();
    let mut previous_seat: isize = 0;
    for &seat_id in seat_ids.iter() {
        if seat_id - previous_seat == 2 {
            print!("My seat ID is {}.\n", seat_id - 1);   
        }
        previous_seat = seat_id;
    }
}
