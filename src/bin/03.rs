use aoc::*;
use std::{collections::HashSet, process};

const DAY: u8 = 3;
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_by(&mut self, direction: char) {
        match direction {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '<' => self.x -= 1,
            '>' => self.x += 1,
            _ => unreachable!(),
        }
    }

    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn part_one(input: &str) -> usize {
    let mut santa_position = Position::new();
    let mut visited_positions = HashSet::new();
    visited_positions.insert(santa_position.get_position());

    for char_pos in input.chars() {
        santa_position.move_by(char_pos);
        visited_positions.insert(santa_position.get_position());
    }
    visited_positions.len()
}

pub fn part_two(input: &str) -> usize {
    let mut santa_position = Position::new();
    let mut robot_santa_position = Position::new();
    let mut santa_turn = true;
    let mut visited_positions = HashSet::new();
    visited_positions.insert(santa_position.get_position());

    for char_pos in input.chars() {
        if santa_turn {
            santa_position.move_by(char_pos);
            visited_positions.insert(santa_position.get_position());
        } else {
            robot_santa_position.move_by(char_pos);
            visited_positions.insert(robot_santa_position.get_position());
        }
        santa_turn = !santa_turn;
    }
    visited_positions.len()
}

fn main() {
    match read_input_file("inputs/03.txt") {
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
    fn part_one_example0() {
        assert_eq!(part_one(""), 1);
    }

    #[test]
    fn part_one_example1() {
        assert_eq!(part_one(">"), 2);
    }

    #[test]
    fn part_one_example2() {
        assert_eq!(part_one("^>v<"), 4);
    }

    #[test]
    fn part_one_example3() {
        assert_eq!(part_one("^v^v^v^v^v"), 2);
    }

    #[test]
    fn part_two_example0() {
        assert_eq!(part_two(""), 1);
    }

    #[test]
    fn part_two_example1() {
        assert_eq!(part_two("^v"), 3);
    }

    #[test]
    fn part_two_example2() {
        assert_eq!(part_two("^>v<"), 3);
    }

    #[test]
    fn part_two_example3() {
        assert_eq!(part_two("^v^v^v^v^v"), 11);
    }
}
