use aoc::*;
use md5::{self};
use std::process;

const DAY: u8 = 4;

pub fn part_one(input: &str) -> u32 {
    compute(input, "00000")
}

fn compute(input: &str, prefix: &str) -> u32 {
    let mut i = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, i));
        let hash = format!("{:x}", hash);
        if hash.starts_with(prefix) {
            return i;
        }
        i += 1;
    }
}

pub fn part_two(input: &str) -> u32 {
    compute(input, "000000")
}

fn main() {
    match read_input_file("inputs/04.txt") {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example1() {
        assert_eq!(part_one("abcdef"), 609043);
    }

    #[test]
    fn part_one_example2() {
        assert_eq!(part_one("pqrstuv"), 1048970);
    }
}
