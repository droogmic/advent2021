use std::collections::btree_map::BTreeMap;
use std::format;
use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;

#[derive(Debug, Default)]
pub struct PartOutput<O> {
    pub answer: O,
}

pub struct DayCalc<D, O> {
    pub parse: fn(&str) -> D,
    pub part1: fn(&D) -> PartOutput<O>,
    pub part2: fn(&D) -> PartOutput<O>,
}

pub struct Day<D, O> {
    pub title: &'static str,
    pub display: (&'static str, &'static str),
    pub calc: DayCalc<D, O>,
    pub example: &'static str,
}

pub trait Printable {
    fn get_display(&self) -> (&'static str, &'static str);
}

impl<D, O> Printable for Day<D, O> {
    fn get_display(&self) -> (&'static str, &'static str) {
        self.display
    }
}

pub trait Calculable {
    // fn part1(&self) -> String;
    // fn part2(&self) -> String;
    fn both(&self, input: &str) -> (String, String);
}

impl<D, O: std::fmt::Display> Calculable for Day<D, O> {
    fn both(&self, input: &str) -> (String, String) {
        let input = (self.calc.parse)(&input.to_string());
        (
            (self.calc.part1)(&input).answer.to_string(),
            (self.calc.part2)(&input).answer.to_string(),
        )
    }
}

pub trait DayTrait: Printable + Calculable + Send {}

impl<D, O: std::fmt::Display> DayTrait for Day<D, O> {}

pub fn get_days() -> BTreeMap<usize, Box<dyn DayTrait + 'static>> {
    let mut days: BTreeMap<usize, Box<dyn DayTrait + 'static>> = BTreeMap::new();
    days.insert(1, Box::new(day01::DAY));
    days.insert(2, Box::new(day02::DAY));
    days.insert(3, Box::new(day03::DAY));
    days
}

pub fn get_input(day: usize) -> String {
    match fs::read_to_string(format!("inputs/day{:02}.txt", day))
        .or_else(|_| fs::read_to_string(format!("../inputs/day{:02}.txt", day)))
    {
        Err(e) => panic!("Err: {}, inputs/day{:02}.txt", e, day),
        Ok(string) => string,
    }
}
