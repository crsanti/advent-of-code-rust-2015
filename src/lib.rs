use std::env;
use std::fs;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

pub enum RunType {
    All,
    Part(u8),
}

pub fn get_run_type() -> RunType {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => RunType::All,
        Some(day_str) => {
            let day: u8 = day_str.parse().unwrap_or(1);
            RunType::Part(day)
        }
    }
}

pub fn read_input_file(input_path: &str) -> std::io::Result<String> {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("src").join(input_path);
    fs::read_to_string(filepath)
}

#[macro_export]
macro_rules! solve_day {
    ($input:expr, $part_fn:ident, $part_num:expr) => {{
        use aoc::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
        use std::fmt::Display;
        use std::time::Instant;

        fn print_result<T: Display>(func: impl FnOnce(&str) -> T, input: &str) {
            let timer = Instant::now();
            let result = func(input);
            let time = timer.elapsed();
            println!(
                "{} {}(elapsed: {:.2?}){}",
                result, ANSI_ITALIC, time, ANSI_RESET
            );
        }

        println!("🎄 {}Part {}{} 🎄", ANSI_BOLD, $part_num, ANSI_RESET);
        println!("");
        print_result($part_fn, $input);
        println!("");
    }};
}
