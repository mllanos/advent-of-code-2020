use std::env;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

fn print_results(title: &str, run: fn()) {
    println!("========================================================================");
    println!("{}", title);
    println!("========================================================================");
    run();
    println!("========================================================================\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage: cargo run %day[1-25] %part[1-2]");
    };

    let day: &str = &args[1];
    let part: &str = &args[2];

    match (day, part) {
        ("1", "1") => print_results("Day 1 - Report Repair - Part 1", day_1::report_repair_part_1::run),
        ("1", "2") => print_results("Day 1 - Report Repair - Part 2", day_1::report_repair_part_2::run),
        ("2", "1") => print_results("Day 2 - Password Philosophy - Part 1", day_2::password_philosophy_part_1::run),
        ("2", "2") => print_results("Day 2 - Password Philosophy - Part 2", day_2::password_philosophy_part_2::run),
        ("3", "1") => print_results("Day 3 - Toboggan Trajectory - Part 1", day_3::toboggan_trajectory_part_1::run),
        ("3", "2") => print_results("Day 3 - Toboggan Trajectory - Part 2", day_3::toboggan_trajectory_part_2::run),
        ("4", "1") => print_results("Day 4 - Passport Processing - Part 1", day_4::passport_processing_part_1::run),
        ("4", "2") => print_results("Day 4 - Passport Processing - Part 2", day_4::passport_processing_part_2::run),
        ("5", "1") => print_results("Day 5 - Binary Boarding - Part 1", day_5::binary_boarding_part_1::run),
        ("5", "2") => print_results("Day 5 - Binary Boarding - Part 2", day_5::binary_boarding_part_2::run),
        ("6", "1") => print_results("Day 6 - Custom Customs - Part 1", day_6::custom_customs_day_1::run),
        (_, _) => println!("The specified case (day {}, part {}) doesn't exist.", day, part)
    }
}
