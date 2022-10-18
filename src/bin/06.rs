use aoc::*;
use std::process;

const DAY: u8 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    action: Action,
    start: (u16, u16),
    end: (u16, u16),
}

#[derive(Debug)]
struct BooleanMatrix {
    matrix: [[bool; 1000]; 1000],
}

impl BooleanMatrix {
    pub fn new() -> Self {
        BooleanMatrix {
            matrix: [[false; 1000]; 1000],
        }
    }
    pub fn apply_instruction(&mut self, instruction: Instruction) {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                match instruction.action {
                    Action::TurnOn => self.matrix[x as usize][y as usize] = true,
                    Action::TurnOff => self.matrix[x as usize][y as usize] = false,
                    Action::Toggle => {
                        self.matrix[x as usize][y as usize] = !self.matrix[x as usize][y as usize]
                    }
                }
            }
        }
    }
    pub fn count_lights_on(&self) -> u32 {
        self.matrix.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .fold(0, |acc, light| if *light { acc + 1 } else { acc })
        })
    }
}

struct IntMatrix {
    // [[u32; 1000]; 1000] provoca stack overflow
    matrix: Vec<Vec<u32>>,
}

impl IntMatrix {
    pub fn new() -> Self {
        IntMatrix {
            matrix: vec![vec![0; 1000]; 1000],
        }
    }

    pub fn apply_instruction(&mut self, instruction: Instruction) {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                match instruction.action {
                    Action::TurnOn => self.matrix[x as usize][y as usize] += 1,
                    Action::TurnOff => {
                        let value = self.matrix[x as usize][y as usize];
                        let value = u32::checked_sub(value, 1).unwrap_or(0);
                        self.matrix[x as usize][y as usize] = value;
                    }
                    Action::Toggle => self.matrix[x as usize][y as usize] += 2,
                }
            }
        }
    }

    pub fn count_lights_on(&self) -> u32 {
        self.matrix
            .iter()
            .fold(0, |acc, row| acc + row.iter().sum::<u32>())
    }
}

pub fn part_one(input: &str) -> u32 {
    let mut matrix = BooleanMatrix::new();

    input.lines().for_each(|line| {
        let instruction = parse_instruction(line);
        matrix.apply_instruction(instruction);
    });

    matrix.count_lights_on()
}

fn parse_instruction(instruction: &str) -> Instruction {
    let mut parts = instruction.split_whitespace();
    let action = match parts.next() {
        Some("turn") => match parts.next() {
            Some("on") => Action::TurnOn,
            Some("off") => Action::TurnOff,
            _ => panic!("Invalid instruction"),
        },
        Some("toggle") => Action::Toggle,
        _ => panic!("Invalid instruction"),
    };

    let mut coords = parts.next().unwrap().split(',');
    let pre_x1 = coords.next().unwrap();
    let pre_y1 = coords.next().unwrap();
    let x1 = pre_x1.parse::<u16>().unwrap();
    let y1 = pre_y1.parse::<u16>().unwrap();

    parts.next();

    let mut coords = parts.next().unwrap().split(',');
    let pre_x2 = coords.next().unwrap();
    let pre_y2 = coords.next().unwrap();
    let x2 = pre_x2.parse::<u16>().unwrap();
    let y2 = pre_y2.parse::<u16>().unwrap();

    Instruction {
        action,
        start: (x1, y1),
        end: (x2, y2),
    }
}

pub fn part_two(input: &str) -> u32 {
    let mut matrix = IntMatrix::new();

    input.lines().for_each(|line| {
        let instruction = parse_instruction(line);
        matrix.apply_instruction(instruction);
    });

    matrix.count_lights_on()
}

fn main() {
    match read_input_file("inputs/06.txt") {
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
        assert_eq!(part_one("turn on 0,0 through 999,999"), 1000 * 1000);
    }

    #[test]
    fn part_one_example2() {
        assert_eq!(part_one("toggle 0,0 through 999,0"), 1000);
    }

    #[test]
    fn part_one_example3() {
        assert_eq!(
            part_one("turn on 0,0 through 999,999\nturn off 499,499 through 500,500"),
            (1000 * 1000) - 4
        );
    }

    #[test]
    fn part_two_example1() {
        assert_eq!(part_two("turn on 0,0 through 0,0"), 1);
    }

    #[test]
    fn part_two_example2() {
        assert_eq!(part_two("toggle 0,0 through 999,999"), 2_000_000);
    }
}
