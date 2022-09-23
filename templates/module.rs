use aoc::*;
use std::process;

const DAY: u8 = DAYNUM;

pub fn part_one(_input: &str) -> u32 {
    0
}

pub fn part_two(_input: &str) -> u32 {
    0
}

fn main() {
    match read_input_file("inputs/0DAYNUM.txt") {
        Ok(input) => match get_run_type() {
            RunType::All => {
                solve_day!(&input, part_one, 1);
                solve_day!(&input, part_two, 2);
            }
            RunType::Part(1) => {
                solve_day!(&input, part_one, 1);
            }
            RunType::Part(2) => {
                solve_day!(&input, part_two, 2);
            }
            RunType::Part(part) => {
                println!("Unknown part {}", part);
                process::exit(1);
            }
        },
        Err(_) => {
            println!("Could not read input file for day {}", DAY);
            process::exit(1);
        }
    }
}
