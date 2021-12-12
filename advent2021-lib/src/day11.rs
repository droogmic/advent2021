use std::collections::HashMap;
use std::collections::HashSet;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone, Default)]
pub struct EnergyLevels {
    map: HashMap<(isize, isize), usize>,
}

impl std::fmt::Debug for EnergyLevels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.map.keys().max_by_key(|pos| pos.0).unwrap().0;
        let max_y = self.map.keys().max_by_key(|pos| pos.1).unwrap().1;
        for y in 0..max_y {
            for x in 0..max_x {
                write!(f, "{}", self.map.get(&(x, y)).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse(input: &str) -> ParseResult<EnergyLevels> {
    let mut map: HashMap<(isize, isize), usize> = HashMap::new();
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, num) in row.chars().enumerate() {
            map.insert(
                (col_idx.try_into().unwrap(), row_idx.try_into().unwrap()),
                num.to_string().parse().map_err(ParseError::Int)?,
            );
        }
    }
    Ok(EnergyLevels { map })
}

pub fn flash_step(energy_levels: &mut EnergyLevels) -> usize {
    let mut flashed: HashSet<(isize, isize)> = HashSet::new();
    let mut to_flash_stack: Vec<(isize, isize)> = vec![];
    for (_, energy_level) in energy_levels.map.iter_mut() {
        *energy_level += 1;
    }
    loop {
        log::debug!("energy_levels:\n{:#?}", energy_levels);
        for (pos, energy_level) in energy_levels.map.iter_mut() {
            if *energy_level > 9 && !flashed.contains(pos) {
                to_flash_stack.push(*pos);
            }
        }
        if to_flash_stack.is_empty() {
            break;
        }
        while let Some(to_flash) = to_flash_stack.pop() {
            flashed.insert(to_flash);
            for x in -1..=1 {
                for y in -1..=1 {
                    if let Some(energy_level) =
                        energy_levels.map.get_mut(&(to_flash.0 + x, to_flash.1 + y))
                    {
                        *energy_level += 1;
                    }
                }
            }
        }
    }
    log::debug!("energy_levels:\n{:#?}", energy_levels);
    for pos in &flashed {
        *energy_levels.map.get_mut(pos).unwrap() = 0
    }
    log::debug!("energy_levels:\n{:#?}", energy_levels);
    flashed.len()
}

pub fn flash_steps(energy_levels: &EnergyLevels, n: usize) -> usize {
    let mut energy_levels = energy_levels.clone();
    (0..n)
        .map(|_| flash_step(&mut energy_levels))
        .sum::<usize>()
}

pub fn part1(energy_levels: &EnergyLevels) -> PartOutput<usize> {
    let energy_levels = energy_levels.clone();
    PartOutput {
        answer: flash_steps(&energy_levels, 100),
    }
}

pub fn part2(energy_levels: &EnergyLevels) -> PartOutput<usize> {
    let full_flash: usize = energy_levels.map.len();
    let mut energy_levels = energy_levels.clone();
    let mut step = 1;
    loop {
        let flashes = flash_step(&mut energy_levels);
        log::debug!("{}=={}", flashes, full_flash);
        if flashes == full_flash {
            break;
        }
        step += 1;
    }
    PartOutput { answer: step }
}

pub const DAY: Day<EnergyLevels, usize> = Day {
    title: "Dumbo Octopus",
    display: (
        "There were {answer} flashes after 100 steps",
        "The first step during which all octopuses flash is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day11.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_step1() {
        let mut energy_levels = parse(DAY.example).unwrap();
        let flashes = flash_step(&mut energy_levels);
        assert_eq!(flashes, 0);
    }

    #[test]
    fn test_example_step2() {
        let mut energy_levels = parse(DAY.example).unwrap();
        let _ = flash_step(&mut energy_levels);
        let flashes = flash_step(&mut energy_levels);
        assert_eq!(flashes, 35);
    }

    #[test]
    fn test_example_2steps() {
        let energy_levels = parse(DAY.example).unwrap();
        let flashes = flash_steps(&energy_levels, 2);
        assert_eq!(flashes, 35);
    }

    #[test]
    fn test_example_part1() {
        let energy_levels = parse(DAY.example).unwrap();
        let flashes = flash_steps(&energy_levels, 100);
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn test_main() {
        let energy_levels = parse(&get_input(11)).unwrap();
        assert_eq!(part1(&energy_levels).answer.to_string(), "1735");
        assert_eq!(part2(&energy_levels).answer.to_string(), "400");
    }
}
