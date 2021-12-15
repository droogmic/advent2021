use ndarray::prelude::*;
use std::collections::HashMap;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Element(u8);

impl From<&Element> for usize {
    fn from(element: &Element) -> Self {
        element.0.into()
    }
}

impl From<&Element> for char {
    fn from(element: &Element) -> Self {
        char::from_u32(u32::from('A') + u32::from(element.0)).unwrap()
    }
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl Element {
    fn from_char(c: char) -> Self {
        let offset = u32::from(c) - u32::from('A');
        Self(offset.try_into().unwrap())
    }
}

#[derive(Debug)]
pub struct PolymerManual {
    template: Vec<Element>,
    pair_insertion: Array2<usize>,
}

const DIM: usize = ('Z' as usize - 'A' as usize) + 1;

impl std::str::FromStr for PolymerManual {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (template_str, pair_insertion_rules_str) = s
            .split_once("\n\n")
            .ok_or_else(|| ParseError::Str(s.to_owned()))?;
        let template = template_str.chars().map(Element::from_char).collect();
        let pair_insertion_rules = pair_insertion_rules_str
            .lines()
            .map(|line| {
                let (left, right) = line
                    .split_once(" -> ")
                    .ok_or_else(|| ParseError::Str(line.to_owned()))?;
                assert_eq!(right.len(), 1);
                Ok((
                    (
                        Element::from_char(left.chars().next().unwrap()),
                        Element::from_char(left.chars().last().unwrap()),
                    ),
                    Element::from_char(right.chars().next().unwrap()),
                ))
            })
            .collect::<ParseResult<HashMap<(Element, Element), Element>>>()?;
        let pair_insertion = {
            let mut pair_insertion = Array2::zeros((DIM * DIM, DIM * DIM));
            for ((from_left, from_right), insert) in pair_insertion_rules.iter() {
                let from: usize = DIM * usize::from(from_left) + usize::from(from_right);
                let to_left: usize = DIM * usize::from(from_left) + usize::from(insert);
                let to_right: usize = DIM * usize::from(insert) + usize::from(from_right);
                pair_insertion[[to_left, from]] = 1;
                pair_insertion[[to_right, from]] = 1;
            }
            pair_insertion
        };
        Ok(Self {
            template,
            pair_insertion,
        })
    }
}

pub fn parse(input: &str) -> ParseResult<PolymerManual> {
    input.parse()
}

fn step(
    pairs: &Array2<usize>,
    counts: Array2<usize>,
    first_last: (&Element, &Element),
) -> Array2<usize> {
    log::debug!("step: {:?}", array_to_counts(&counts, first_last));
    pairs.dot(&counts)
}

fn steps(
    pairs: &Array2<usize>,
    counts: Array2<usize>,
    n: usize,
    first_last: (&Element, &Element),
) -> Array2<usize> {
    (0..n).fold(counts, |acc, _| step(pairs, acc, first_last))
}

fn array_to_counts(
    arr_counts: &Array2<usize>,
    first_last: (&Element, &Element),
) -> HashMap<Element, usize> {
    let mut counts = HashMap::new();
    for row in 0..arr_counts.nrows() {
        let additional: usize = arr_counts[[row, 0]];
        if additional > 0 {
            let left: u8 = (row / DIM).try_into().unwrap();
            let right: u8 = (row % DIM).try_into().unwrap();
            counts
                .entry(Element(left))
                .and_modify(|count| *count += additional)
                .or_insert(additional);
            counts
                .entry(Element(right))
                .and_modify(|count| *count += additional)
                .or_insert(additional);
        }
    }
    log::trace!("counts: {:?}", counts);
    for (element, count) in counts.iter_mut() {
        log::trace!("{:?}, {:?}, {:?}", element, count, first_last);
        if first_last.0 == element {
            *count += 1
        }
        if first_last.1 == element {
            *count += 1
        }
        *count /= 2;
    }
    log::trace!("counts: {:?}", counts);
    counts
}

fn template_to_array(template: &[Element]) -> Array2<usize> {
    let count_indices: Vec<usize> = template
        .windows(2)
        .map(|window| {
            DIM * usize::from(window.first().unwrap()) + usize::from(window.last().unwrap())
        })
        .collect();
    let mut arr_counts = Array2::zeros((DIM * DIM, 1));
    for count_idx in count_indices {
        arr_counts[[count_idx, 0]] += 1;
    }
    arr_counts
}

pub fn get_counts(
    pair_insertion: &Array2<usize>,
    template: Vec<Element>,
    n: usize,
    first_last: (&Element, &Element),
) -> HashMap<Element, usize> {
    let arr_counts = template_to_array(&template);
    log::debug!("init: {:?}", array_to_counts(&arr_counts, first_last));
    let arr_counts = steps(pair_insertion, arr_counts, n, first_last);
    array_to_counts(&arr_counts, first_last)
}

pub fn part1(manual: &PolymerManual) -> PartOutput<usize> {
    let first_last = (
        manual.template.first().unwrap(),
        manual.template.last().unwrap(),
    );
    let counts = get_counts(
        &manual.pair_insertion,
        manual.template.clone(),
        10,
        first_last,
    );
    log::debug!("final: {:?}", counts);
    PartOutput {
        answer: counts.values().max().unwrap() - counts.values().min().unwrap(),
    }
}

pub fn part2(manual: &PolymerManual) -> PartOutput<usize> {
    let first_last = (
        manual.template.first().unwrap(),
        manual.template.last().unwrap(),
    );
    let counts = get_counts(
        &manual.pair_insertion,
        manual.template.clone(),
        40,
        first_last,
    );
    log::debug!("final: {:?}", counts);
    PartOutput {
        answer: counts.values().max().unwrap() - counts.values().min().unwrap(),
    }
}

pub const DAY: Day<PolymerManual, usize> = Day {
    title: "Extended Polymerization",
    display: (
        "Difference of most and least after 10 steps is {answer}",
        "Difference of most and least after 40 steps is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day14.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_template() {
        let template = vec![
            Element::from_char('N'),
            Element::from_char('N'),
            Element::from_char('C'),
            Element::from_char('B'),
        ];
        let first_last = (template.first().unwrap(), template.last().unwrap());
        let counts = array_to_counts(&template_to_array(&template), first_last);
        assert_eq!(counts.get(&Element::from_char('N')).unwrap(), &2);
    }

    #[test]
    fn test_template_ends() {
        let template = vec![
            Element::from_char('N'),
            Element::from_char('N'),
            Element::from_char('C'),
            Element::from_char('N'),
        ];
        let first_last = (template.first().unwrap(), template.last().unwrap());
        let counts = array_to_counts(&template_to_array(&template), first_last);
        assert_eq!(counts.get(&Element::from_char('N')).unwrap(), &3);
    }

    #[test]
    fn test_example_part1() {
        let manual = parse(DAY.example).unwrap();
        let first_last = (
            manual.template.first().unwrap(),
            manual.template.last().unwrap(),
        );
        log::trace!("manual: {:?}", manual);
        let counts = get_counts(
            &manual.pair_insertion,
            manual.template.clone(),
            10,
            first_last,
        );
        log::debug!("counts: {:?}", counts);
        assert_eq!(counts.get(&Element::from_char('B')).unwrap(), &1749);
    }

    #[test]
    fn test_example_part2() {
        let manual = parse(DAY.example).unwrap();
        let first_last = (
            manual.template.first().unwrap(),
            manual.template.last().unwrap(),
        );
        log::trace!("manual: {:?}", manual);
        let counts = get_counts(
            &manual.pair_insertion,
            manual.template.clone(),
            40,
            first_last,
        );
        log::debug!("counts: {:?}", counts);
        assert_eq!(
            counts.get(&Element::from_char('B')).unwrap(),
            &2192039569602
        );
        assert_eq!(counts.get(&Element::from_char('H')).unwrap(), &3849876073);
    }

    #[test]
    fn test_main() {
        let manual = parse(&get_input(14)).unwrap();
        assert_eq!(part1(&manual).answer.to_string(), "3048");
        assert_eq!(part2(&manual).answer.to_string(), "3288891573057");
    }
}
