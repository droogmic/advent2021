use std::collections::HashMap;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone, Default, Debug)]
pub struct Heightmap {
    map: HashMap<(usize, usize), usize>,
}

pub fn parse(input: &str) -> ParseResult<Heightmap> {
    let mut map = HashMap::new();
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, num) in row.chars().enumerate() {
            map.insert(
                (col_idx, row_idx),
                num.to_string().parse().map_err(ParseError::Int)?,
            );
        }
    }
    Ok(Heightmap { map })
}

pub fn get_risk(heightmap: &Heightmap) -> usize {
    let mut low_points: Vec<(usize, usize)> = vec![];
    for (pos, val) in heightmap.map.iter() {
        if val
            >= heightmap
                .map
                .get(&(pos.0.wrapping_sub(1), pos.1))
                .unwrap_or(&usize::MAX)
            || val
                >= heightmap
                    .map
                    .get(&(pos.0 + 1, pos.1))
                    .unwrap_or(&usize::MAX)
            || val
                >= heightmap
                    .map
                    .get(&(pos.0, pos.1.wrapping_sub(1)))
                    .unwrap_or(&usize::MAX)
            || val
                >= heightmap
                    .map
                    .get(&(pos.0, pos.1 + 1))
                    .unwrap_or(&usize::MAX)
        {
            continue;
        }
        log::debug!("low point: {:?}", pos);
        low_points.push(*pos);
    }
    low_points
        .iter()
        .map(|point| heightmap.map.get(point).unwrap() + 1)
        .sum::<usize>()
}

pub fn basin_walk(heightmap: &Heightmap) -> usize {
    let mut basins: HashMap<(usize, usize), usize> = HashMap::new();
    for (pos, _) in heightmap.map.iter() {
        let mut walk = *pos;
        loop {
            let from_height = heightmap.map.get(&(walk.0, walk.1)).unwrap();
            if *from_height == 9 {
                break;
            }
            log::debug!("walk from {:?} at {:?}", walk, from_height);
            // N
            if from_height
                >= heightmap
                    .map
                    .get(&(walk.0, walk.1.wrapping_sub(1)))
                    .unwrap_or(&usize::MAX)
            {
                walk = (walk.0, walk.1 - 1);
                continue;
            }
            // W
            if from_height
                >= heightmap
                    .map
                    .get(&(walk.0.wrapping_sub(1), walk.1))
                    .unwrap_or(&usize::MAX)
            {
                walk = (walk.0 - 1, walk.1);
                continue;
            }
            // S (strict)
            if from_height
                > heightmap
                    .map
                    .get(&(walk.0, walk.1 + 1))
                    .unwrap_or(&usize::MAX)
            {
                walk = (walk.0, walk.1 + 1);
                continue;
            }
            // E (strict)
            if from_height
                > heightmap
                    .map
                    .get(&(walk.0 + 1, walk.1))
                    .unwrap_or(&usize::MAX)
            {
                walk = (walk.0 + 1, walk.1);
                continue;
            }
            break;
        }
        basins
            .entry(walk)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut sizes: Vec<usize> = basins.iter().map(|(_basin, size)| *size).collect();
    sizes.sort_unstable();
    log::debug!("basins: {:?}", basins);
    sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap()
}

pub fn part1(heightmap: &Heightmap) -> PartOutput<usize> {
    PartOutput {
        answer: get_risk(heightmap),
    }
}

pub fn part2(heightmap: &Heightmap) -> PartOutput<usize> {
    PartOutput {
        answer: basin_walk(heightmap),
    }
}

pub const DAY: Day<Heightmap, usize> = Day {
    title: "Smoke Basin",
    display: (
        "The sum of the risk levels of all low points on the heightmap is {answer}",
        "The product of the three largest basins is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day09.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_part1() {
        let heightmap = parse(DAY.example).unwrap();
        let risk = get_risk(&heightmap);
        assert_eq!(risk, 15);
    }

    #[test]
    fn test_example_part2() {
        let heightmap = parse(DAY.example).unwrap();
        let result = basin_walk(&heightmap);
        assert_eq!(result, 1134);
    }

    #[test]
    fn test_main() {
        let something = parse(&get_input(9)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "600");
        assert_eq!(part2(&something).answer.to_string(), "987840");
    }
}
