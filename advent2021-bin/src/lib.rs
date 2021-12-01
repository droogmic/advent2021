use std::format;
use std::fs;

fn get_string(day: usize) -> String {
    match fs::read_to_string(format!("inputs/day{:02}.txt", day)) {
        Err(e) => panic!("Err: {}, inputs/day{:02}.txt", e, day),
        Ok(string) => string,
    }
}