use aoc::*;
use std::cmp::min;
use std::process;

const DAY: u8 = 2;

#[derive(Debug)]
pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(present: &str) -> Self {
        let mut sizes = present.split('x').take(3).map(|x| {
            x.parse::<u32>()
                .expect(&format!("{} is not a positive number", x.to_string()))
        });

        Present {
            length: sizes.next().expect("Present has no length"),
            width: sizes.next().expect("Present has no width"),
            height: sizes.next().expect("Present has no height"),
        }
    }

    fn paper(&self) -> u32 {
        let side1 = self.length * self.width;
        let side2 = self.length * self.height;
        let side3 = self.height * self.width;
        let required_paper = 2 * side1 + 2 * side2 + 2 * side3;
        let extra_paper = min(side1, min(side2, side3));
        required_paper + extra_paper
    }

    fn ribbon(&self) -> u32 {
        let s1 = min(self.width, self.height);
        let s2 = min(self.width, self.length);
        let ribbon_for_present = s1 * 2 + s2 * 2;
        let ribbon_for_bow = self.length * self.width * self.height;
        ribbon_for_present + ribbon_for_bow
    }
}

pub fn part_one(input: &str) -> u32 {
    input.lines().map(|x| Present::new(x).paper()).sum()
}

pub fn part_two(input: &str) -> u32 {
    input.lines().map(|x| Present::new(x).ribbon()).sum()
}

fn main() {
    match read_input_file("inputs/02.txt") {
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
    fn part1_example1() {
        assert_eq!(Present::new("2x3x4").paper(), 58);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(Present::new("1x1x10").paper(), 43);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(Present::new("2x3x4").ribbon(), 34);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(Present::new("1x1x10").ribbon(), 14);
    }
}
