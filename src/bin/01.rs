use aoc::*;
use std::process;

const DAY: u8 = 1;

fn part_one(input: &str) -> i32 {
    input.chars().fold(0, |total, c| total + num_from_char(&c))
}

fn part_two(input: &str) -> u32 {
    input
        .char_indices()
        .try_fold(0, |mut total, (index, c)| {
            total += num_from_char(&c);
            if total == -1 {
                Err((index as u32) + 1)
            } else {
                Ok(total)
            }
        })
        .unwrap_err()
}

fn num_from_char(c: &char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn main() {
    match read_input_file("inputs/01.txt") {
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
        assert_eq!(part_one("(())"), 0);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part_one("()()"), 0);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part_one("((("), 3);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part_one("(()(()("), 3);
    }

    #[test]
    fn part1_example5() {
        assert_eq!(part_one("))((((("), 3);
    }

    #[test]
    fn part1_example6() {
        assert_eq!(part_one("())"), -1);
    }

    #[test]
    fn part1_example7() {
        assert_eq!(part_one("))("), -1);
    }

    #[test]
    fn part1_example8() {
        assert_eq!(part_one(")))"), -3);
    }

    #[test]
    fn part1_example9() {
        assert_eq!(part_one(")())())"), -3);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part_two(")"), 1);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part_two("()())"), 5);
    }
}
