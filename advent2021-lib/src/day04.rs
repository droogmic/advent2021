use ndarray::{Array2, Axis};
use std::collections::VecDeque;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone, Debug)]
pub enum Marked {
    Marked,
    Unmarked,
}

impl Default for Marked {
    fn default() -> Self {
        Marked::Unmarked
    }
}

#[derive(Clone, Default, Debug)]
pub struct Number {
    val: usize,
    mark: Marked,
}

#[derive(Clone, Debug)]
pub struct Board(Array2<Number>);

impl Board {
    pub fn is_bingo(&self) -> bool {
        (0..self.0.ndim()).any(|dim| {
            self.0.axis_iter(Axis(dim)).any(|col_view| {
                col_view
                    .iter()
                    .all(|num| matches!(num.mark, Marked::Marked))
            })
        })
    }

    pub fn score(&self, number: usize) -> usize {
        let score = self
            .0
            .iter()
            .filter_map(|num| match num.mark {
                Marked::Marked => None,
                Marked::Unmarked => Some(num.val),
            })
            .sum::<usize>()
            * number;
        log::debug!("unmarked numbers: {:#?}", self.0.map(|num| num.val));
        log::debug!("last number: {:#?}", number);
        log::debug!("score: {:#?}", score);
        score
    }
}

pub struct Bingo {
    pub numbers: Vec<usize>,
    pub boards: Vec<Board>,
}

pub fn get_bingo(input: &str) -> ParseResult<Bingo> {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .ok_or(ParseError {})?
        .split(',')
        .map(|element| element.parse().map_err(|_| ParseError))
        .collect::<ParseResult<Vec<usize>>>()?;
    log::debug!("numbers: {:?}", numbers);
    let board_lines = lines
        .skip_while(|&line| line.is_empty())
        .collect::<Vec<&str>>();
    let mut board_lines = board_lines.split(|line| line.is_empty()).peekable();
    let board_size = {
        let &first = board_lines.peek().ok_or(ParseError)?;
        log::debug!("first board: {:#?}", first);
        Ok((
            first.len(),
            first
                .iter()
                .next()
                .ok_or(ParseError)?
                .split_whitespace()
                .count(),
        ))
    }?;
    log::debug!("board size: {:?}", board_size);
    let boards = board_lines
        .map(|lines| {
            let mut board = Board(Array2::default(board_size));
            // TODO: not this
            for (row, line) in lines.iter().enumerate() {
                for (col, val) in line.split_whitespace().enumerate() {
                    match val.parse() {
                        Ok(val) => {
                            board.0[[row, col]] = Number {
                                val,
                                mark: Marked::Unmarked,
                            }
                        }
                        Err(_) => return Err(ParseError),
                    }
                }
            }
            Ok(board)
        })
        .collect::<ParseResult<Vec<Board>>>()?;
    log::debug!("boards: {:#?}", boards);
    Ok(Bingo { numbers, boards })
}

pub struct BingoResult {
    first: (usize, Board),
    last: (usize, Board),
}

// TODO: take non-ref
pub fn play(bingo: &Bingo) -> BingoResult {
    let Bingo { numbers, boards } = bingo;
    let mut bingo_boards = VecDeque::<(usize, Board)>::new();
    let mut boards: Vec<Board> = boards.clone();
    for n in numbers {
        log::debug!("number: {:#?}", n);
        for board in &mut boards {
            board.0.map_inplace(|num: &mut Number| {
                if &num.val == n {
                    num.mark = Marked::Marked;
                }
            });
        }
        let (new_bingo_boards, remaining_boards) =
            boards.into_iter().partition(|board| board.is_bingo());
        boards = remaining_boards;
        bingo_boards.extend(new_bingo_boards.into_iter().map(|board| (*n, board)));
    }
    BingoResult {
        first: bingo_boards.pop_front().unwrap(),
        last: bingo_boards.pop_back().unwrap(),
    }
}

pub fn parse_and_play(input: &str) -> ParseResult<BingoResult> {
    let bingo = get_bingo(input)?;
    Ok(play(&bingo))
}

pub fn part1(bingo_result: &BingoResult) -> PartOutput<usize> {
    let BingoResult {
        first: (number, board),
        last: _last,
    } = bingo_result;
    PartOutput {
        answer: board.score(*number),
    }
}

pub fn part2(bingo_result: &BingoResult) -> PartOutput<usize> {
    let BingoResult {
        first: _first,
        last: (number, board),
    } = bingo_result;
    PartOutput {
        answer: board.score(*number),
    }
}

pub const DAY: Day<BingoResult, usize> = Day {
    title: "Giant Squid",
    display: (
        "The final score of the winning bingo board is {answer}",
        "The final score of the worst bingo board is {answer}",
    ),
    calc: DayCalc {
        parse: parse_and_play,
        part1,
        part2,
    },
    example: include_str!("../examples/day04.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_part1() {
        let bingo = get_bingo(DAY.example).unwrap();
        let result = play(&bingo);
        let (number, board) = result.first;
        assert_eq!(board.score(number), 4512);
    }

    #[test]
    fn test_example_part2() {
        let bingo = get_bingo(DAY.example).unwrap();
        let result = play(&bingo);
        let (number, board) = result.last;
        assert_eq!(board.score(number), 1924);
    }

    #[test]
    fn test_main() {
        let bingo = get_bingo(&get_input(4)).unwrap();
        let result = play(&bingo);
        assert_eq!(part1(&result).answer.to_string(), "63552");
        assert_eq!(part2(&result).answer.to_string(), "9020");
    }
}
