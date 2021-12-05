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
        let diff_x = i128::try_from(line.end_x).unwrap() - i128::try_from(line.start_x).unwrap();
        let step_x: isize = diff_x.signum().try_into().unwrap();
        let min_y = std::cmp::min(line.start_y, line.end_y);
        let max_y = std::cmp::max(line.start_y, line.end_y);
        let diff_y = i128::try_from(line.end_y).unwrap() - i128::try_from(line.start_y).unwrap();
        let step_y: isize = diff_y.signum().try_into().unwrap();
        if !diag && step_x != 0 && step_y != 0 {
            continue;
        }
        for p in 0..=std::cmp::max(max_x - min_x, max_y - min_y) {
            vents
                .0
                .entry((
                    line.start_x
                        .checked_add_signed(step_x.checked_mul(p.try_into().unwrap()).unwrap())
                        .unwrap(),
                    line.start_y
                        .checked_add_signed(step_y.checked_mul(p.try_into().unwrap()).unwrap())
                        .unwrap(),
                ))
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
