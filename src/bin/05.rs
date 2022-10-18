use aoc::*;
use std::process;

const DAY: u8 = 5;

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let mut vowels = 0;
            let mut has_double = false;
            let mut has_bad = false;
            let mut last = ' ';
            for c in line.chars() {
                if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                    vowels += 1;
                }
                if c == last {
                    has_double = true;
                }
                if (last == 'a' && c == 'b')
                    || (last == 'c' && c == 'd')
                    || (last == 'p' && c == 'q')
                    || (last == 'x' && c == 'y')
                {
                    has_bad = true;
                }
                last = c;
            }
            vowels >= 3 && has_double && !has_bad
        })
        .count() as u32
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let mut has_pair = false;
            let mut has_repeat = false;

            for i in 0..line.len() - 1 {
                let pair = &line[i..i + 2];
                if line[i + 2..].contains(pair) {
                    has_pair = true;
                }
                if i < line.len() - 2 {
                    let c1 = line.chars().nth(i).unwrap();
                    let c2 = line.chars().nth(i + 2).unwrap();
                    if c1 == c2 {
                        has_repeat = true;
                    }
                }
            }
            has_pair && has_repeat
        })
        .count() as u32
}

fn main() {
    match read_input_file("inputs/05.txt") {
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
        assert_eq!(part_one("ugknbfddgicrmopn"), 1);
    }

    #[test]
    fn part_one_example2() {
        assert_eq!(part_one("aaa"), 1);
    }

    #[test]
    fn part_one_example3() {
        assert_eq!(part_one("jchzalrnumimnmhp"), 0);
    }

    #[test]
    fn part_one_example4() {
        assert_eq!(part_one("haegwjzuvuyypxyu"), 0);
    }

    #[test]
    fn part_one_example5() {
        assert_eq!(part_one("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn part_two_example1() {
        assert_eq!(part_two("qjhvhtzxzqqjkmpb"), 1);
    }

    #[test]
    fn part_two_example2() {
        assert_eq!(part_two("xxyxx"), 1);
    }

    #[test]
    fn part_two_example3() {
        assert_eq!(part_two("uurcxstgmygtbstg"), 0);
    }

    #[test]
    fn part_two_example4() {
        assert_eq!(part_two("ieodomkazucvgmuy"), 0);
    }
}
