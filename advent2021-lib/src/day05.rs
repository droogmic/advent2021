use std::collections::HashMap;

use recap::Recap;
use serde::Deserialize;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"^(?P<start_x>\d+),(?P<start_y>\d+) -> (?P<end_x>\d+),(?P<end_y>\d+)$"#)]
pub struct Line {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

pub struct Lines(Vec<Line>);

#[derive(Clone, Default)]
pub struct Vents(HashMap<(usize, usize), usize>);

impl std::fmt::Debug for Vents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.0.keys().min_by_key(|pos| pos.0).unwrap().0;
        let max_x = self.0.keys().max_by_key(|pos| pos.0).unwrap().0;
        let min_y = self.0.keys().min_by_key(|pos| pos.1).unwrap().1;
        let max_y = self.0.keys().max_by_key(|pos| pos.1).unwrap().1;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                write!(
                    f,
                    "{}",
                    self.0
                        .get(&(x, y))
                        .map_or(".".to_owned(), |v| v.to_string())
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Vents {
    fn count_overlap(&self) -> usize {
        self.0.values().filter(|&&v| v >= 2).count()
    }
}

pub fn parse(input: &str) -> ParseResult<Lines> {
    Ok(Lines(
        input
            .lines()
            .map(|line| line.parse().map_err(|_| ParseError::Str(line.to_owned())))
            .collect::<ParseResult<_>>()?,
    ))
}

pub fn plot(lines: &Lines, diag: bool) -> Vents {
    let mut vents = Vents(HashMap::new());
    for line in &lines.0 {
        let min_x = std::cmp::min(line.start_x, line.end_x);
        let max_x = std::cmp::max(line.start_x, line.end_x);
        let diff_x = max_x - min_x;
        let min_y = std::cmp::min(line.start_y, line.end_y);
        let max_y = std::cmp::max(line.start_y, line.end_y);
        let diff_y = max_y - min_y;
        let (start, end, get_key) = if diff_y == 0 {
            (
                min_x,
                max_x,
                Box::new(|point| (point, line.start_y)) as Box<dyn Fn(usize) -> (usize, usize)>,
            )
        } else if diff_x == 0 {
            (
                min_y,
                max_y,
                Box::new(|point| (line.start_x, point)) as Box<dyn Fn(usize) -> (usize, usize)>,
            )
        } else if !diag && diff_x == diff_y {
            continue;
        } else if diag && diff_x == diff_y {
            let asc_x = line.start_x == min_x;
            let asc_y = line.start_y == min_y;
            (
                0,
                diff_x,
                if asc_x ^ asc_y {
                    Box::new(|p| (min_x + p, max_y - p)) as Box<dyn Fn(usize) -> (usize, usize)>
                } else {
                    Box::new(|p| (min_x + p, min_y + p)) as Box<dyn Fn(usize) -> (usize, usize)>
                },
            )
        } else {
            panic!("unrecognised line direction")
        };
        for p in start..=end {
            vents
                .0
                .entry(get_key(p))
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        log::debug!("incrementing {:?}\n{:#?}", line, vents);
    }
    vents
}

pub fn part1(lines: &Lines) -> PartOutput<usize> {
    let vents = plot(lines, false);
    PartOutput {
        answer: vents.count_overlap(),
    }
}

pub fn part2(lines: &Lines) -> PartOutput<usize> {
    let vents = plot(lines, true);
    PartOutput {
        answer: vents.count_overlap(),
    }
}

pub const DAY: Day<Lines, usize> = Day {
    title: "Hydrothermal Venture",
    display: (
        "There are {answer} dangerous areas with at least two vertical or horizontal lines overlapping",
        "There are {answer} dangerous areas with at least two lines overlapping",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day05.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_part1() {
        let vents = plot(&parse(DAY.example).unwrap(), false);
        assert_eq!(vents.count_overlap(), 5);
    }

    #[test]
    fn test_example_part2() {
        let vents = plot(&parse(DAY.example).unwrap(), true);
        assert_eq!(vents.count_overlap(), 12);
    }

    #[test]
    fn test_main() {
        let something = parse(&get_input(5)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "4873");
        assert_eq!(part2(&something).answer.to_string(), "19472");
    }
}
