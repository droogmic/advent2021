use std::str::FromStr;
use std::ops::Add;

use crate::{Day, Parts};

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
            _ => return Err(CommandError{}),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CommandError;

pub struct Command {
    direction: Direction,
    distance: usize,
}

#[derive(Default)]
pub struct Location {
    position: usize,
    depth: usize
}

impl Add<&Command> for Location {
    type Output = Self;

    fn add(self, command: &Command) -> Self {
        match command {
            Command{ direction: Direction::Forward, distance } => Self {
                position: self.position + distance,
                depth: self.depth,
            },
            Command{ direction: Direction::Down, distance } => Self {
                position: self.position,
                depth: self.depth + distance,
            },
            Command{ direction: Direction::Up, distance } => Self {
                position: self.position,
                depth: self.depth - distance,
            },
        }
        
    }
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

pub fn get_data(input: String) -> Vec<Command> {
    input
        .lines()
        .map(|line| line.parse().expect("bad input"))
        .collect()
}

pub fn navigate(commands: &[Command]) -> Location {
    commands.iter().fold(Location::default(), |location, command| location + command)
}

pub fn main(input: String) -> Day {
    let commands = get_data(input);
    let location = navigate(&commands);
    let product = location.position * location.depth;

    Day {
        answers: Parts(product.to_string(), product.to_string()),
        display: Parts(
            format!(
                "The horizontal position to final depth product is {}",
                product
            ),
            format!(
                "There are {} sums larger than the previous sum",
                product
            ),
        ),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_string;

    #[test]
    fn test_example_part1() {
        let result = navigate(
            &get_data("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2".into()),
        );
        assert_eq!(result, 7);
    }

    #[test]
    fn test_example_part2() {
        let result = navigate(
            &get_data("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2".into()),
        );
        assert_eq!(result, 5);
    }

    #[test]
    fn test_main() {
        let day = main(get_string(1));
        assert_eq!(day.answers.0, "0");
        assert_eq!(day.answers.1, "0");
    }
}
