use std::collections::btree_map::BTreeMap;
use std::format;
use std::fs;

pub mod day01;
pub mod day02;

#[derive(Debug, Default)]
pub struct Parts(pub String, pub String);

#[derive(Debug, Default)]
pub struct Day {
    pub answers: Parts,
    pub display: Parts,
    pub visual: Option<String>,
}

pub fn get_days() -> BTreeMap<usize, fn(String) -> Day> {
    let mut days = BTreeMap::<usize, fn(_) -> _>::new();
    days.insert(1, day01::main);
    days.insert(2, day02::main);
    days
}

pub fn get_string(day: usize) -> String {
    match fs::read_to_string(format!("inputs/day{:02}.txt", day)) {
        Err(e) => panic!("Err: {}, inputs/day{:02}.txt", e, day),
        Ok(string) => string,
    }
}
