use std::collections::HashSet;

use recap::Recap;
use serde::Deserialize;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FoldDirection {
    X,
    Y,
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"^fold along (?P<direction>.+)=(?P<location>.+)$"#)]
pub struct Fold {
    direction: FoldDirection,
    location: usize,
}

#[derive(Clone)]
pub struct Dots(HashSet<(usize, usize)>);

#[derive(Debug)]
pub struct TransparentPaper {
    dots: Dots,
    folds: Vec<Fold>,
}

impl std::fmt::Debug for Dots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.0.iter().max_by_key(|pos| pos.0).unwrap().0;
        let max_y = self.0.iter().max_by_key(|pos| pos.1).unwrap().1;
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.0.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::str::FromStr for TransparentPaper {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dot_str, fold_str) = s.split_once("\n\n").ok_or(ParseError::Empty)?;
        let dots = dot_str
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(',').ok_or(ParseError::Empty)?;
                Ok((
                    left.parse().map_err(|_| ParseError::Str(left.to_owned()))?,
                    right
                        .parse()
                        .map_err(|_| ParseError::Str(right.to_owned()))?,
                ))
            })
            .collect::<ParseResult<HashSet<(usize, usize)>>>()?;
        let folds = fold_str
            .lines()
            .map(|line| line.parse().map_err(|_| ParseError::Str(line.to_owned())))
            .collect::<ParseResult<Vec<Fold>>>()?;
        Ok(Self {
            dots: Dots(dots),
            folds,
        })
    }
}

pub fn parse(input: &str) -> ParseResult<TransparentPaper> {
    input.parse()
}

pub fn apply_fold(dots: &mut Dots, fold: &Fold) {
    log::debug!("applying {:?} on dots:\n{:#?}", fold, dots);
    match fold.direction {
        FoldDirection::X => {
            let dots_to_fold: Vec<(usize, usize)> = dots
                .0
                .iter()
                .filter(|dot| dot.0 > fold.location)
                .cloned()
                .collect();
            for dot_to_fold in dots_to_fold {
                dots.0.remove(&dot_to_fold);
                dots.0.insert((
                    fold.location - (dot_to_fold.0 - fold.location),
                    dot_to_fold.1,
                ));
            }
        }
        FoldDirection::Y => {
            let dots_to_fold: Vec<(usize, usize)> = dots
                .0
                .iter()
                .filter(|dot| dot.1 > fold.location)
                .cloned()
                .collect();
            for dot_to_fold in dots_to_fold {
                dots.0.remove(&dot_to_fold);
                dots.0.insert((
                    dot_to_fold.0,
                    fold.location - (dot_to_fold.1 - fold.location),
                ));
            }
        }
    }
}

pub fn apply_folds(paper: &TransparentPaper) -> Dots {
    let mut dots = paper.dots.clone();
    for fold in &paper.folds {
        apply_fold(&mut dots, fold);
    }
    dots
}

// TODO, allow returning a usize
pub fn part1(paper: &TransparentPaper) -> PartOutput<String> {
    let mut dots = paper.dots.clone();
    let fold = paper.folds.first().unwrap();
    apply_fold(&mut dots, fold);
    PartOutput {
        answer: dots.0.len().to_string(),
    }
}

pub fn part2(paper: &TransparentPaper) -> PartOutput<String> {
    let dots = apply_folds(paper);
    PartOutput {
        answer: format!("The code is\n{:#?}", dots),
    }
}

pub const DAY: Day<TransparentPaper, String> = Day {
    title: "Transparent Origami",
    display: ("{answer} dots are visible after one fold.", "{answer}"),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day13.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_part1() {
        let paper = parse(DAY.example).unwrap();
        let mut dots = paper.dots.clone();
        let fold = paper.folds.first().unwrap();
        apply_fold(&mut dots, fold);
        assert_eq!(dots.0.len(), 17);
    }

    #[test]
    fn test_main() {
        let something = parse(&get_input(13)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "814");
    }
}
