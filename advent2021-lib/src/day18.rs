use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone, Debug, PartialEq)]
pub enum SnailfishElement {
    Value(usize),
    Pair(Box<SnailfishNumber>),
}

impl std::str::FromStr for SnailfishElement {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some('[') = s.chars().next() {
            Self::Pair(Box::new(s.parse()?))
        } else {
            Self::Value(s.parse().map_err(ParseError::Int)?)
        })
    }
}

impl SnailfishElement {
    pub fn explode(&mut self, nesting: usize) -> Option<(Option<usize>, Option<usize>)> {
        match self {
            Self::Value(_val) => None,
            Self::Pair(number) => number.explode(nesting + 1),
        }
    }

    // Pop pop
    pub fn magnitude(&self) -> usize {
        match self {
            Self::Value(val) => *val,
            Self::Pair(number) => number.magnitude(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnailfishNumber(SnailfishElement, SnailfishElement);

impl std::str::FromStr for SnailfishNumber {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(matches!(s.chars().next(), Some('[')));
        let (elements, nesting) = s.chars().skip(1).fold(
            (vec![String::new()], Some(0usize)),
            |(mut s, nesting), c| {
                if let Some(nesting) = nesting {
                    s.last_mut().unwrap().push(c);
                    match c {
                        '[' => (s, Some(nesting + 1)),
                        ']' => {
                            if nesting > 0 {
                                (s, Some(nesting - 1))
                            } else {
                                s.last_mut().unwrap().pop();
                                (s, None)
                            }
                        }
                        ',' => {
                            if nesting == 0 {
                                s.last_mut().unwrap().pop();
                                s.push(String::new());
                            }
                            (s, Some(nesting))
                        }
                        '0'..='9' => (s, Some(nesting)),
                        _ => panic!(),
                    }
                } else {
                    s.push("Big Ooof".to_owned());
                    (s, None)
                }
            },
        );
        if nesting.is_some() {
            return Err(ParseError::Str("parsing unclosed".to_owned()));
        }
        if elements.last().unwrap() == "Big Ooof" {
            return Err(ParseError::Str("parsing extra".to_owned()));
        }
        if elements.len() != 2 {
            return Err(ParseError::Str("parsing not pair".to_owned()));
        }
        let number = Self(elements[0].parse()?, elements[1].parse()?);
        log::trace!("{:?} -> {:?}", (&elements, &nesting), number);
        Ok(number)
    }
}

impl std::ops::Add for SnailfishNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut combined = SnailfishNumber(
            SnailfishElement::Pair(Box::new(self)),
            SnailfishElement::Pair(Box::new(other)),
        );
        loop {
            if combined.explode(0).is_some() {
                continue;
            }
            if combined.split() {
                continue;
            }
            break combined;
        }
    }
}

impl SnailfishNumber {
    fn get_pair(&self) -> Option<(usize, usize)> {
        if let SnailfishElement::Value(left) = self.0 {
            if let SnailfishElement::Value(right) = self.1 {
                return Some((left, right));
            }
        }
        None
    }

    pub fn explode(&mut self, nesting: usize) -> Option<(Option<usize>, Option<usize>)> {
        log::trace!("explode {} {:?}", nesting, self);
        if nesting >= 4 {
            if let Some(pair) = self.get_pair() {
                return Some((Some(pair.0), Some(pair.1)));
            }
        }
        if let Some(pair) = self.0.explode(nesting) {
            if let Some(right) = pair.1 {
                if let Some(_left) = pair.0 {
                    // the entire exploding pair is replaced with the regular number 0
                    self.0 = SnailfishElement::Value(0);
                }
                // pair's right value is added to the first regular number to the right of the exploding pair (if any)
                match &mut self.1 {
                    SnailfishElement::Value(val) => self.1 = SnailfishElement::Value(*val + right),
                    SnailfishElement::Pair(number) => number.add_left(right),
                };
                // pair's left value
                return Some((pair.0, None));
            }
            return Some((pair.0, pair.1));
        }
        if let Some(pair) = self.1.explode(nesting) {
            if let Some(left) = pair.0 {
                if let Some(_right) = pair.1 {
                    // the entire exploding pair is replaced with the regular number 0
                    self.1 = SnailfishElement::Value(0);
                }
                // pair's left value is added to the first regular number to the left of the exploding pair
                match &mut self.0 {
                    SnailfishElement::Value(val) => self.0 = SnailfishElement::Value(*val + left),
                    SnailfishElement::Pair(number) => number.add_right(left),
                };
                // pair's right value
                return Some((None, pair.1));
            }
            return Some((pair.0, pair.1));
        }
        None
    }

    fn add_left(&mut self, add: usize) {
        match &mut self.0 {
            SnailfishElement::Value(val) => self.0 = SnailfishElement::Value(*val + add),
            SnailfishElement::Pair(number) => number.add_left(add),
        };
    }

    fn add_right(&mut self, add: usize) {
        match &mut self.1 {
            SnailfishElement::Value(val) => self.1 = SnailfishElement::Value(*val + add),
            SnailfishElement::Pair(number) => number.add_right(add),
        };
    }

    pub fn split(&mut self) -> bool {
        log::trace!("split {:?}", self);
        (match &mut self.0 {
            SnailfishElement::Value(val) => {
                if *val >= 10 {
                    let left = val.checked_div(2).unwrap();
                    let right = val.checked_sub(left).unwrap();
                    self.0 = SnailfishElement::Pair(Box::new(Self(
                        SnailfishElement::Value(left),
                        SnailfishElement::Value(right),
                    )));
                    true
                } else {
                    false
                }
            }
            SnailfishElement::Pair(number) => number.split(),
        } || match &mut self.1 {
            SnailfishElement::Value(val) => {
                if *val >= 10 {
                    let left = val.checked_div(2).unwrap();
                    let right = val.checked_sub(left).unwrap();
                    self.1 = SnailfishElement::Pair(Box::new(Self(
                        SnailfishElement::Value(left),
                        SnailfishElement::Value(right),
                    )));
                    true
                } else {
                    false
                }
            }
            SnailfishElement::Pair(number) => number.split(),
        })
    }

    // Pop pop
    pub fn magnitude(&self) -> usize {
        3 * self.0.magnitude() + 2 * self.1.magnitude()
    }
}

#[derive(Debug)]
pub struct Homework(Vec<SnailfishNumber>);

impl Homework {
    pub fn sum(&self) -> SnailfishNumber {
        self.0
            .iter()
            .cloned()
            .reduce(|acc, next| acc + next)
            .unwrap()
    }

    pub fn max_pair_magnititude(&self) -> usize {
        let mut max = 0;
        for (left_idx, left) in self.0.iter().enumerate() {
            for (right_idx, right) in self.0.iter().enumerate() {
                if left_idx == right_idx {
                    continue;
                }
                let magnitude = (left.clone() + right.clone()).magnitude();
                if magnitude > max {
                    max = magnitude;
                }
            }
        }
        max
    }
}

impl std::str::FromStr for Homework {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Homework(
            s.lines()
                .map(|line| line.parse())
                .collect::<ParseResult<Vec<SnailfishNumber>>>()?,
        ))
    }
}

pub fn parse(input: &str) -> ParseResult<Homework> {
    input.parse()
}

pub fn part1(homework: &Homework) -> PartOutput<usize> {
    PartOutput {
        answer: homework.sum().magnitude(),
    }
}

pub fn part2(homework: &Homework) -> PartOutput<usize> {
    PartOutput {
        answer: homework.max_pair_magnititude(),
    }
}

pub const DAY: Day<Homework, usize> = Day {
    title: "Snailfish",
    display: (
        "The magnitude of the final sum is {answer}",
        "The largest magnitude of any sum of two different snailfish numbers is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day18.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_parse_1() {
        let pair: SnailfishNumber = "[1,2]".parse().unwrap();
        assert_eq!(pair.magnitude(), 7);
    }

    #[test]
    fn test_parse_2() {
        let pair: SnailfishNumber = "[[1,2],3]".parse().unwrap();
        assert_eq!(pair.magnitude(), 27);
    }

    #[test]
    fn test_parse_3() {
        let pair: SnailfishNumber = "[[1,2],[[3,4],5]]".parse().unwrap();
        assert_eq!(pair.magnitude(), 143);
    }

    #[test]
    fn test_parse_4() {
        let pair: SnailfishNumber = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        assert_eq!(pair.magnitude(), 1384);
    }

    #[test]
    fn test_parse_5() {
        let pair: SnailfishNumber = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap();
        assert_eq!(pair.magnitude(), 445);
    }

    #[test]
    fn test_parse_6() {
        let pair: SnailfishNumber = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap();
        assert_eq!(pair.magnitude(), 791);
    }

    #[test]
    fn test_parse_7() {
        let pair: SnailfishNumber = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap();
        assert_eq!(pair.magnitude(), 1137);
    }

    #[test]
    fn test_parse_8() {
        let pair: SnailfishNumber = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .unwrap();
        assert_eq!(pair.magnitude(), 3488);
    }

    #[test]
    fn test_parse_bad() {
        let pair: ParseResult<SnailfishNumber> = "[[1,2],[[3,4],5]".parse();
        assert!(pair.is_err());
        let pair: ParseResult<SnailfishNumber> = "[[1,2],[[3,4],5]]]".parse();
        assert!(pair.is_err());
    }

    #[test]
    fn test_explode_1() {
        let mut pair: SnailfishNumber = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        pair.explode(0);
        assert_eq!(pair, "[[[[0,9],2],3],4]".parse().unwrap());
    }

    #[test]
    fn test_explode_2() {
        let mut pair: SnailfishNumber = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        pair.explode(0);
        assert_eq!(pair, "[7,[6,[5,[7,0]]]]".parse().unwrap());
    }

    #[test]
    fn test_explode_3() {
        let mut pair: SnailfishNumber = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
        pair.explode(0);
        assert_eq!(pair, "[[6,[5,[7,0]]],3]".parse().unwrap());
    }

    #[test]
    fn test_explode_4() {
        let mut pair: SnailfishNumber = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
        pair.explode(0);
        assert_eq!(pair, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap());
    }

    #[test]
    fn test_explode_5() {
        let mut pair: SnailfishNumber = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
        pair.explode(0);
        assert_eq!(pair, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse().unwrap());
    }

    #[test]
    fn test_split() {
        let mut pair: SnailfishNumber = "[[[[0,7],4],[15,[0,13]]],[1,1]]".parse().unwrap();
        pair.split();
        assert_eq!(pair, "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".parse().unwrap());
        pair.split();
        assert_eq!(
            pair,
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".parse().unwrap()
        );
    }

    #[test]
    fn test_add() {
        let first: SnailfishNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let second: SnailfishNumber = "[1,1]".parse().unwrap();
        assert_eq!(
            first + second,
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap()
        );
    }

    #[test]
    fn test_sum() {
        let homework: Homework = parse("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]").unwrap();
        assert_eq!(
            homework.sum(),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap()
        );
    }

    #[test]
    fn test_example_part1() {
        let homework: Homework = parse(DAY.example).unwrap();
        let sum = homework.sum();
        assert_eq!(
            sum,
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
                .parse()
                .unwrap()
        );
        assert_eq!(sum.magnitude(), 4140);
    }

    #[test]
    fn test_example_part2() {
        let homework: Homework = parse(DAY.example).unwrap();
        let magnitude = homework.max_pair_magnititude();
        assert_eq!(magnitude, 3993);
    }

    #[test]
    fn test_main() {
        let homework = parse(&get_input(18)).unwrap();
        assert_eq!(part1(&homework).answer.to_string(), "3734");
        assert_eq!(part2(&homework).answer.to_string(), "4837");
    }
}
