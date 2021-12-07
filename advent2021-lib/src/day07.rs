use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone, Default, Debug)]
pub struct CrabPositions(pub Vec<usize>);

pub fn parse(input: &str) -> ParseResult<CrabPositions> {
    Ok(CrabPositions(
        input
            .trim()
            .split(',')
            .map(|val| val.parse().map_err(ParseError::Int))
            .collect::<ParseResult<_>>()?,
    ))
}

pub fn get_const_fuel(positions: &CrabPositions) -> usize {
    let CrabPositions(mut positions) = positions.clone();
    // If the length is even, we get 2 medians. Both have the same fuel cost.
    let median_idx = positions.len() / 2;
    positions.select_nth_unstable(median_idx);
    let median = positions[median_idx];
    positions
        .iter()
        .fold(0, |acc, val| acc + val.abs_diff(median))
}

fn get_fuel(positions: &[usize], target: usize) -> usize {
    positions.iter().fold(0, |acc, val| {
        acc + (1..=val.abs_diff(target)).fold(0, |tot, off| tot + off)
    })
}

pub fn get_linear_fuel(positions: &CrabPositions) -> usize {
    let CrabPositions(positions) = positions.clone();
    // Probably an OK starting point
    let mean = (positions.iter().sum::<usize>() + positions.len() / 2) / positions.len();
    let mut best_pos = mean;
    let mut best_fuel = get_fuel(&positions, best_pos);
    // search up
    loop {
        let next_fuel = get_fuel(&positions, best_pos + 1);
        if next_fuel > best_fuel {
            break;
        }
        best_fuel = next_fuel;
        best_pos = best_pos + 1;
    }
    //search down
    loop {
        let next_fuel = get_fuel(&positions, best_pos - 1);
        if next_fuel > best_fuel {
            break;
        }
        best_fuel = next_fuel;
        best_pos = best_pos + 1;
    }
    best_fuel
}

pub fn part1(positions: &CrabPositions) -> PartOutput<usize> {
    let fuel = get_const_fuel(positions);
    PartOutput { answer: fuel }
}

pub fn part2(positions: &CrabPositions) -> PartOutput<usize> {
    let fuel = get_linear_fuel(positions);
    PartOutput { answer: fuel }
}

pub const DAY: Day<CrabPositions, usize> = Day {
    title: "The Treachery of Whales",
    display: (
        "The crab used {answer} fuel to align",
        "Foobar foobar foobar {answer}",
    ),
    calc: DayCalc {
        parse: parse,
        part1,
        part2,
    },
    example: "16,1,2,0,4,2,7,1,2,14",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_part1() {
        let positions = parse(DAY.example).unwrap();
        let result = get_const_fuel(&positions);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_example_part2() {
        let positions = parse(DAY.example).unwrap();
        let result = get_linear_fuel(&positions);
        assert_eq!(result, 168);
    }

    #[test]
    fn test_main() {
        let positions = parse(&get_input(7)).unwrap();
        assert_eq!(part1(&positions).answer.to_string(), "355764");
        assert_eq!(part2(&positions).answer.to_string(), "99634572");
    }
}
