use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone, Debug)]
pub struct Lines(Vec<String>);

pub fn parse(input: &str) -> ParseResult<Lines> {
    Ok(Lines(input.lines().map(|line| line.to_owned()).collect()))
}

#[derive(Clone, Debug)]
enum ChunkPair {
    Paren,
    Square,
    Curly,
    Angle,
}

impl ChunkPair {
    fn syntax_score(&self) -> usize {
        match self {
            Self::Paren => 3,
            Self::Square => 57,
            Self::Curly => 1197,
            Self::Angle => 25137,
        }
    }
    fn completion_score(&self) -> usize {
        match self {
            Self::Paren => 1,
            Self::Square => 2,
            Self::Curly => 3,
            Self::Angle => 4,
        }
    }
}

#[derive(Clone, Debug)]
enum ChunkPart {
    Open,
    Close,
}

#[derive(Clone, Debug)]
struct Chunk {
    pair: ChunkPair,
    part: ChunkPart,
}

impl std::str::FromStr for Chunk {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "(" => Chunk {
                pair: ChunkPair::Paren,
                part: ChunkPart::Open,
            },
            ")" => Chunk {
                pair: ChunkPair::Paren,
                part: ChunkPart::Close,
            },
            "[" => Chunk {
                pair: ChunkPair::Square,
                part: ChunkPart::Open,
            },
            "]" => Chunk {
                pair: ChunkPair::Square,
                part: ChunkPart::Close,
            },
            "{" => Chunk {
                pair: ChunkPair::Curly,
                part: ChunkPart::Open,
            },
            "}" => Chunk {
                pair: ChunkPair::Curly,
                part: ChunkPart::Close,
            },
            "<" => Chunk {
                pair: ChunkPair::Angle,
                part: ChunkPart::Open,
            },
            ">" => Chunk {
                pair: ChunkPair::Angle,
                part: ChunkPart::Close,
            },
            _ => return Err(ParseError::Empty),
        })
    }
}

pub fn corrupted_score(line: &str) -> Option<usize> {
    log::debug!("corrupted?: {:?}", line);
    let mut heap: Vec<ChunkPair> = vec![];
    for char in line.chars() {
        log::debug!("heap: {:?}", heap);
        let chunk: Chunk = char.to_string().parse().unwrap();
        match chunk.part {
            ChunkPart::Open => {
                heap.push(chunk.pair);
            }
            ChunkPart::Close => {
                let last = heap.pop().unwrap();
                if std::mem::discriminant(&chunk.pair) == std::mem::discriminant(&last) {
                    continue;
                } else {
                    return Some(chunk.pair.syntax_score());
                }
            }
        }
    }
    None
}

pub fn corrupted_scores(lines: &Lines) -> usize {
    lines
        .0
        .iter()
        .filter_map(|line| corrupted_score(line))
        .sum()
}

pub fn incomplete_score(line: &str) -> usize {
    log::debug!("incomplete: {:?}", line);
    let mut heap: Vec<ChunkPair> = vec![];
    for char in line.chars() {
        log::debug!("heap: {:?}", heap);
        let chunk: Chunk = char.to_string().parse().unwrap();
        match chunk.part {
            ChunkPart::Open => {
                heap.push(chunk.pair);
            }
            ChunkPart::Close => {
                let last = heap.pop().unwrap();
                if std::mem::discriminant(&chunk.pair) == std::mem::discriminant(&last) {
                    continue;
                } else {
                    panic!("Corrupted");
                }
            }
        }
    }
    heap.into_iter()
        .rev()
        .fold(0, |acc, pair| (acc * 5) + pair.completion_score())
}

pub fn incomplete_scores(lines: &Lines) -> usize {
    let mut scores: Vec<usize> = lines
        .0
        .iter()
        .filter(|line| corrupted_score(line).is_none())
        .map(|line| incomplete_score(line))
        .collect();
    let median_idx = scores.len() / 2;
    scores.select_nth_unstable(median_idx);
    scores[median_idx]
}

pub fn part1(lines: &Lines) -> PartOutput<usize> {
    PartOutput {
        answer: corrupted_scores(lines),
    }
}

pub fn part2(lines: &Lines) -> PartOutput<usize> {
    PartOutput {
        answer: incomplete_scores(lines),
    }
}

pub const DAY: Day<Lines, usize> = Day {
    title: "Syntax Scoring",
    display: (
        "The total syntax error score is {answer}",
        "The middle completion score is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day10.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_part1() {
        let lines = parse(DAY.example).unwrap();
        let result = corrupted_scores(&lines);
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_example_part2() {
        let lines = parse(DAY.example).unwrap();
        let result = incomplete_scores(&lines);
        assert_eq!(result, 288957);
    }

    #[test]
    fn test_main() {
        let something = parse(&get_input(10)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "462693");
        assert_eq!(part2(&something).answer.to_string(), "3094671161");
    }
}
