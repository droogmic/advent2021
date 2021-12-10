use std::collections::HashMap;
use std::collections::HashSet;

use recap::Recap;
use serde::Deserialize;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"^(?P<observations>.+) \| (?P<output>.+)$"#)]
pub struct DisplayObservationsParse {
    observations: String,
    output: String,
}

#[derive(Clone, Default, Debug)]
pub struct Digit {
    segments: Vec<char>,
}

impl std::str::FromStr for Digit {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Digit {
            segments: s.chars().collect(),
        })
    }
}

impl std::convert::From<Digit> for HashSet<usize> {
    fn from(digit: Digit) -> Self {
        let mut values = HashSet::new();
        for val in digit.segments.into_iter().map(|segment| match segment {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => panic!("unknown segment"),
        }) {
            values.insert(val);
        }
        values
    }
}

#[derive(Clone, Default, Debug)]
pub struct Digits(Vec<Digit>);

impl std::str::FromStr for Digits {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Digits(
            s.split(' ')
                .map(|digit| digit.parse().map_err(|_| ParseError::Str(digit.to_owned())))
                .collect::<ParseResult<_>>()?,
        ))
    }
}

struct Wires(Vec<usize>);

impl From<&Wires> for Vec<char> {
    fn from(wires: &Wires) -> Self {
        wires
            .0
            .iter()
            .map(|wire| match wire {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                _ => panic!("bad wire"),
            })
            .collect()
    }
}

impl std::fmt::Debug for Wires {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Vec::<char>::from(self)
                .into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[derive(Clone, Default, Debug)]
pub struct DisplayObservation {
    observations: Digits,
    output: Digits,
}

impl std::convert::TryFrom<DisplayObservationsParse> for DisplayObservation {
    type Error = ParseError;
    fn try_from(parsed: DisplayObservationsParse) -> Result<Self, Self::Error> {
        Ok(Self {
            observations: parsed.observations.parse()?,
            output: parsed.output.parse()?,
        })
    }
}

impl std::str::FromStr for DisplayObservation {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s
            .parse::<DisplayObservationsParse>()
            .map_err(|_| ParseError::Str(s.to_owned()))?;
        DisplayObservation::try_from(parsed)
    }
}

#[derive(Clone, Default, Debug)]
pub struct DisplayObservations(Vec<DisplayObservation>);

pub fn parse(input: &str) -> ParseResult<DisplayObservations> {
    Ok(DisplayObservations(
        input
            .lines()
            .map(|line| line.parse())
            .collect::<ParseResult<Vec<DisplayObservation>>>()?,
    ))
}

fn number_to_segments(number: usize) -> HashSet<usize> {
    match number {
        0 => HashSet::from([0, 1, 2, 4, 5, 6]),
        1 => HashSet::from([2, 5]),
        2 => HashSet::from([0, 2, 3, 4, 6]),
        3 => HashSet::from([0, 2, 3, 5, 6]),
        4 => HashSet::from([1, 2, 3, 5]),
        5 => HashSet::from([0, 1, 3, 5, 6]),
        6 => HashSet::from([0, 1, 3, 4, 5, 6]),
        7 => HashSet::from([0, 2, 5]),
        8 => HashSet::from([0, 1, 2, 3, 4, 5, 6]),
        9 => HashSet::from([0, 1, 2, 3, 5, 6]),
        n => panic!("unexpected number {}", n),
    }
}

const fn segment_count_to_number(number: usize) -> Option<usize> {
    match number {
        2 => Some(1), // 1
        3 => Some(7), // 7
        4 => Some(4), // 4
        7 => Some(8), // 8
        _ => None,
    }
}

pub fn count_1s_4s_7s_8s(observations: &DisplayObservations) -> usize {
    observations
        .0
        .iter()
        .map(|display| {
            display
                .output
                .0
                .iter()
                .filter_map(|digit| segment_count_to_number(digit.segments.len()))
        })
        .flatten()
        .count()
}

pub fn get_outputs(display_observations: &DisplayObservations) -> Vec<usize> {
    display_observations
        .0
        .iter()
        .map(|display| {
            let measurements = {
                let mut m: Vec<Digit> = display.observations.0.clone();
                m.extend(display.output.0.clone());
                m
            };
            let wire_to_segments = {
                let mut wire_to_segments = HashMap::new();
                for wire in 0..7 {
                    wire_to_segments.insert(wire, HashSet::from([0, 1, 2, 3, 4, 5, 6, 7]));
                }
                for measurement in &measurements {
                    let segment_count = measurement.segments.len();
                    let measurement_wires: HashSet<usize> = measurement.clone().into();
                    if let Some(number) = segment_count_to_number(segment_count) {
                        for wire in measurement_wires {
                            wire_to_segments.entry(wire).and_modify(|segments| {
                                *segments = segments
                                    .intersection(&number_to_segments(number))
                                    .cloned()
                                    .collect();
                            });
                        }
                    }
                }
                wire_to_segments
            };
            log::debug!("wire_to_segments: {:?}", wire_to_segments);
            let wire_to_segment_permutations = {
                let mut permutations: Vec<Vec<usize>> = vec![vec![]];
                for wire in 0..7 {
                    let last_layer: Vec<Vec<usize>> = permutations.drain(..).collect();
                    permutations = wire_to_segments
                        .get(&wire)
                        .unwrap()
                        .iter()
                        .map(|&segment: &usize| {
                            last_layer
                                .iter()
                                .filter_map(|p: &Vec<usize>| {
                                    if p.contains(&segment) {
                                        return None;
                                    }
                                    let new_permutation: Vec<usize> =
                                        p.iter().cloned().chain(std::iter::once(segment)).collect();
                                    Some(new_permutation)
                                })
                                .collect::<Vec<Vec<usize>>>()
                        })
                        .flatten()
                        .collect();
                    log::debug!("permutations: {:?}", permutations);
                }
                permutations
            };
            log::debug!(
                "wire_to_segment_permutations: {:?}",
                wire_to_segment_permutations.len()
            );
            let mut best_wire_to_segment: Vec<usize> = vec![];
            for wire_to_segment in wire_to_segment_permutations {
                if measurements.iter().all(|measurement| {
                    let measurement_wires: HashSet<usize> = measurement.clone().into();
                    let measurement_segments: HashSet<usize> = measurement_wires
                        .into_iter()
                        .map(|wire| wire_to_segment[wire])
                        .collect();
                    (0..=9).any(|number| measurement_segments == number_to_segments(number))
                }) {
                    best_wire_to_segment = wire_to_segment;
                    break;
                }
            }
            log::debug!("best_wire_to_segment: {:?}", best_wire_to_segment);
            let output: usize = display
                .output
                .0
                .iter()
                .map(|output| {
                    let output_wires: HashSet<usize> = output.clone().into();
                    let output_segments: HashSet<usize> = output_wires
                        .into_iter()
                        .map(|wire| best_wire_to_segment[wire])
                        .collect();
                    (0..=9)
                        .find(|&number| output_segments == number_to_segments(number))
                        .unwrap()
                })
                .fold((0, 1000), |acc, val| (acc.0 + acc.1 * val, acc.1 / 10))
                .0;
            log::debug!("output: {:?}", output);
            output
        })
        .collect()
}

pub fn part1(display_observations: &DisplayObservations) -> PartOutput<usize> {
    PartOutput {
        answer: count_1s_4s_7s_8s(display_observations),
    }
}

pub fn part2(display_observations: &DisplayObservations) -> PartOutput<usize> {
    PartOutput {
        answer: get_outputs(display_observations).iter().sum::<usize>(),
    }
}

pub const DAY: Day<DisplayObservations, usize> = Day {
    title: "Seven Segment Search",
    display: (
        "Digits 1, 4, 7, or 8 appear {answer} times",
        "The sum of the decoded outputs is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day08.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_part1() {
        let display_observations = parse(DAY.example).unwrap();
        let result = count_1s_4s_7s_8s(&display_observations);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_example_part2() {
        let display_observations = parse(DAY.example).unwrap();
        let result = get_outputs(&display_observations);
        assert_eq!(result.iter().sum::<usize>(), 61229);
    }

    #[test]
    fn test_main() {
        let something = parse(&get_input(8)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "288");
        assert_eq!(part2(&something).answer.to_string(), "940724");
    }
}
