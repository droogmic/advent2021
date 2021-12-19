use recap::Recap;
use serde::Deserialize;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone, Debug)]
pub struct Position(isize, isize);

#[derive(Clone, Debug)]
pub struct Velocity(isize, isize);

pub fn step(position: &Position, velocity: &Velocity) -> (Position, Velocity) {
    (
        Position(position.0 + velocity.0, position.1 + velocity.1),
        Velocity(velocity.0 - velocity.0.signum(), velocity.1 - 1),
    )
}

impl Velocity {
    pub fn steps(&self, n: usize) -> Position {
        let mut position = Position(0, 0);
        let mut velocity = Self(self.0, self.1);
        for _ in 0..n {
            let (new_position, new_velocity) = step(&position, &velocity);
            position = new_position;
            velocity = new_velocity;
        }
        position
    }

    /// Max height is when y = 0
    /// which is when t = y
    /// Therefore max_height = u + (u-1) + (u-2) ...
    pub fn max_height(&self) -> usize {
        (0..=self.1.try_into().unwrap()).sum()
    }
}

///
/// Current assumption:
/// x is positive
/// y is negative
#[derive(Debug, Deserialize, Recap)]
#[recap(
    regex = r#"^target area: x=(?P<start_x>\d+)..(?P<end_x>\d+), y=(?P<start_y>-\d+)..(?P<end_y>-\d+)$"#
)]
pub struct TargetArea {
    start_x: isize,
    end_x: isize,
    start_y: isize,
    end_y: isize,
}

impl TargetArea {
    fn closest_x(&self) -> isize {
        std::cmp::min(self.start_x, self.end_x)
    }

    fn furthest_x(&self) -> isize {
        std::cmp::max(self.start_x, self.end_x)
    }

    fn lowest_y(&self) -> isize {
        std::cmp::min(self.start_y, self.end_y)
    }

    fn check(&self, mut velocity: Velocity) -> bool {
        log::debug!("check: {:?}", velocity);
        let mut position = Position(0, 0);
        while position.0 < self.furthest_x() && position.1 > self.lowest_y() {
            log::trace!("check({:?}, {:?})", position, velocity);
            let (new_position, new_velocity) = step(&position, &velocity);
            if (self.start_x..=self.end_x).contains(&new_position.0)
                && (self.start_y..=self.end_y).contains(&new_position.1)
            {
                return true;
            }
            position = new_position;
            velocity = new_velocity;
        }
        false
    }

    /// Get the minimum initial horizontal speed
    /// this is equal to the time it would take to stop at the closest_x
    /// which is minimising u for (0..=u).sum() >= closest_x
    ///
    fn min_vel_x(&self) -> isize {
        let mut u: isize = self.closest_x() / 2;
        loop {
            let x: isize = (0..=u).sum();
            if x < self.closest_x() {
                break u + 1;
            }
            u -= 1;
        }
    }

    /// Get the maximum initial horizontal speed
    /// this is equal to stepping to furthest_x
    ///
    fn max_vel_x(&self) -> isize {
        self.furthest_x()
    }

    /// Get the maximum initial horizontal speed
    /// this is equal to the time it would take to stop at the end
    /// which is maximising u for (0..=u).sum() <= furthest_x
    ///
    fn furthest_stop_vel_x(&self) -> isize {
        let mut u: isize = 0;
        loop {
            let x: isize = (0..=u).sum();
            if x > self.furthest_x() {
                break u - 1;
            }
            u += 1;
        }
    }

    /// Get minimum vertical (largest negative) speed at launch
    ///
    /// We therefore are looking for a minimum u, where u >= lowest_y
    /// u = lowest_y
    ///
    fn min_vel_y(&self) -> isize {
        self.lowest_y()
    }

    /// Get maximum vertical speed at launch
    ///
    /// Given infinite time,
    /// the probe will always arrive at y=0 at the same speed as it was launched.
    /// The next step will take the probe to u+1
    /// We therefore are looking for a maximum u, where u+1 <= |lowest_y|
    /// u = |lowest_y| - 1
    ///
    fn max_vel_y(&self) -> isize {
        assert!(self.lowest_y() < 0);
        self.lowest_y().checked_neg().unwrap() - 1
    }

    /// Maximum X is used just to give the probe more time in the sweet spot
    fn max_height(&self) -> usize {
        let furthest_stop_vel_x = self.furthest_stop_vel_x();
        let max_vel_y = self.max_vel_y();
        if log::log_enabled!(log::Level::Trace) {
            for incr in 0..5 {
                let u = Velocity(furthest_stop_vel_x + incr, max_vel_y + incr);
                log::trace!(
                    "max_vel: {:?} -> {:?}, {:?}",
                    u,
                    u.steps((u.0).try_into().unwrap()),
                    self.check(u.clone()),
                );
            }
        }
        let vel = Velocity(furthest_stop_vel_x, max_vel_y);
        let height = vel.max_height();
        log::debug!("max height: {:?} -> {:?}", vel, height,);
        if !self.check(vel) {
            panic!();
        }
        height
    }

    /// Maximum X is used just to give the probe more time in the sweet spot
    fn get_velocities(&self) -> Vec<Velocity> {
        let mut velocities = vec![];
        for y in self.min_vel_y()..=self.max_vel_y() {
            for x in self.min_vel_x()..=self.max_vel_x() {
                let vel = Velocity(x, y);
                if self.check(vel.clone()) {
                    velocities.push(vel);
                }
            }
        }
        velocities
    }
}

pub fn parse(input: &str) -> ParseResult<TargetArea> {
    input
        .trim()
        .parse()
        .map_err(|_| ParseError::Str(input.to_owned()))
}

pub fn part1(target_area: &TargetArea) -> PartOutput<usize> {
    PartOutput {
        answer: target_area.max_height(),
    }
}

pub fn part2(target_area: &TargetArea) -> PartOutput<usize> {
    PartOutput {
        answer: target_area.get_velocities().len(),
    }
}

pub const DAY: Day<TargetArea, usize> = Day {
    title: "Trick Shot",
    display: (
        "{answer} is the highest y position it reaches on this trajectory",
        "There are {answer} distinct initial velocity values which reach the target",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: "target area: x=20..30, y=-10..-5",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example() {
        let target_area = TargetArea {
            start_x: 20,
            end_x: 30,
            start_y: -10,
            end_y: -5,
        };
        assert!(target_area.check(Velocity(7, 2)));
        assert!(target_area.check(Velocity(6, 3)));
        assert!(target_area.check(Velocity(9, 0)));
        assert!(!target_area.check(Velocity(17, -4)));
        assert!(target_area.check(Velocity(6, 9)));
    }

    #[test]
    fn test_example_part1() {
        let target_area = parse(DAY.example).unwrap();
        assert_eq!(target_area.max_height(), 45);
    }

    #[test]
    fn test_example_part2() {
        let target_area = parse(DAY.example).unwrap();
        assert_eq!(target_area.get_velocities().len(), 112);
    }

    #[test]
    fn test_main() {
        let something = parse(&get_input(17)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "5886");
        assert_eq!(part2(&something).answer.to_string(), "1806");
    }
}
