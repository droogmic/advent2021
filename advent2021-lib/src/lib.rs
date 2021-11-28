use std::collections::btree_map::BTreeMap;

pub mod day00;

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
    days.insert(0, day00::main);
    days
}