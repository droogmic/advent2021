use std::collections::btree_map::BTreeMap;
use std::format;
use std::fs;

pub mod day01;
pub mod day02;

#[derive(Debug, Default)]
pub struct Parts(pub String, pub String);

#[derive(Debug, Default)]
pub struct DayOutput {
    pub title: String,
    pub answers: Parts,
    pub display: Parts,
    pub visual: Option<String>,
}

#[derive(Debug)]
pub struct Day {
    pub title: String,
    pub calc: fn(String) -> DayOutput,
}

pub fn get_days() -> BTreeMap<usize, Day> {
    let mut days = BTreeMap::new();
    days.insert(1, Day {
        title: "".to_owned(),
        calc: day01::main,
    });
    days.insert(2, Day {
        title: "Dive!".to_owned(),
        calc: day02::main,
    });
    days
}

pub fn get_input(day: usize) -> String {
    match fs::read_to_string(format!("inputs/day{:02}.txt", day)).or_else(|_| {
        fs::read_to_string(format!("../inputs/day{:02}.txt", day))
    }) {
        Err(e) => panic!("Err: {}, inputs/day{:02}.txt", e, day),
        Ok(string) => string,
    }
}
