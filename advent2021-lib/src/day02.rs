use std::ops::Add;
use std::str::FromStr;

use crate::{Day, DayCalc, PartOutput};

#[derive(Debug, Clone)]
struct DirectionError;

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => return Err(CommandError {}),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CommandError;

pub struct Command {
    direction: Direction,
    distance: usize,
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        Ok(Command {
            direction: tokens.next().ok_or(CommandError)?.parse()?,
            distance: tokens
                .next()
                .ok_or(CommandError)?
                .parse()
                .map_err(|_| CommandError {})?,
        })
    }
}

#[derive(Default)]
pub struct Location {
    position: usize,
    depth: usize,
}

impl Add<&Command> for Location {
    type Output = Self;

    fn add(self, command: &Command) -> Self {
        match command {
            Command {
                direction: Direction::Forward,
                distance,
            } => Self {
                position: self.position + distance,
                depth: self.depth,
            },
            Command {
                direction: Direction::Down,
                distance,
            } => Self {
                position: self.position,
                depth: self.depth + distance,
            },
            Command {
                direction: Direction::Up,
                distance,
            } => Self {
                position: self.position,
                depth: self.depth - distance,
            },
        }
    }
}

#[derive(Default)]
pub struct LocationAim {
    position: usize,
    depth: usize,
    aim: usize,
}

impl Add<&Command> for LocationAim {
    type Output = Self;

    fn add(self, command: &Command) -> Self {
        match command {
            Command {
                direction: Direction::Forward,
                distance,
            } => Self {
                position: self.position + distance,
                depth: self.depth + self.aim * distance,
                aim: self.aim,
            },
            Command {
                direction: Direction::Down,
                distance,
            } => Self {
                position: self.position,
                depth: self.depth,
                aim: self.aim + distance,
            },
            Command {
                direction: Direction::Up,
                distance,
            } => Self {
                position: self.position,
                depth: self.depth,
                aim: self.aim - distance,
            },
        }
    }
}

pub fn get_data(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| line.parse().expect("Bad Input"))
        .collect()
}

pub fn navigate(commands: &[Command]) -> Location {
    commands
        .iter()
        .fold(Location::default(), |location, command| location + command)
}

pub fn navigate_aim(commands: &[Command]) -> LocationAim {
    commands
        .iter()
        .fold(LocationAim::default(), |location, command| {
            location + command
        })
}

pub fn part1(commands: &[Command]) -> PartOutput<usize> {
    let location = navigate(commands);
    PartOutput {
        answer: location.position * location.depth,
    }
}
pub fn part2(commands: &[Command]) -> PartOutput<usize> {
    let location = navigate_aim(&commands);
    PartOutput {
        answer: location.position * location.depth,
    }
}

pub const DAY: Day<Vec<Command>, [Command], usize> = Day {
    title: "Dive!",
    display: (
        "The horizontal position to final depth product is {answer}",
        "The horizontal position to final depth product is {answer}",
    ),
    calc: DayCalc {
        parse: get_data,
        part1: part1,
        part2: part2,
    },
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;

    #[test]
    fn test_example_part1() {
        let result = navigate(&get_data(
            "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2".into(),
        ));
        assert_eq!(result.position * result.depth, 150);
    }

    #[test]
    fn test_example_part2() {
        let result = navigate_aim(&get_data(
            "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2".into(),
        ));
        assert_eq!(result.position * result.depth, 900);
    }

    #[test]
    fn test_main() {
        let input = get_data(&get_input(2));
        assert_eq!(part1(&input).answer.to_string(), "2027977");
        assert_eq!(part2(&input).answer.to_string(), "1903644897");
    }
}
