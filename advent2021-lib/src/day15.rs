use std::collections::HashMap;

use pathfinding::prelude::astar_bag;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone)]
pub struct Risk(usize);

impl std::fmt::Debug for Risk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Map(HashMap<(usize, usize), Risk>);

impl Map {
    fn full_map(&mut self) {
        log::trace!("map: {:?}", self.0);
        let width = self.0.keys().max_by_key(|n| n.0).unwrap().0 + 1;
        let height = self.0.keys().max_by_key(|n| n.1).unwrap().1 + 1;
        for tile_x in 0..5 {
            for tile_y in 0..5 {
                for x in 0..width {
                    for y in 0..height {
                        let reference = self.0.get(&(x, y)).unwrap().0;
                        let offset = tile_x + tile_y;
                        let new = Risk(((reference + offset - 1) % 9) + 1);
                        self.0
                            .insert((tile_x * width + x, tile_y * height + y), new);
                    }
                }
            }
        }
        log::trace!("full_map: {:?}", self.0);
    }
}

impl std::str::FromStr for Map {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for (row_idx, row) in s.lines().enumerate() {
            for (col_idx, num) in row.chars().enumerate() {
                map.insert(
                    (col_idx, row_idx),
                    Risk(num.to_string().parse().map_err(ParseError::Int)?),
                );
            }
        }
        log::debug!("map: {:?}", map);
        Ok(Self(map))
    }
}

pub fn parse(input: &str) -> ParseResult<Map> {
    input.parse()
}

pub fn path(map: &Map) -> usize {
    type Node = (usize, usize);
    type Cost = usize;
    let start: Node = (0, 0);
    let end = map.0.keys().max_by_key(|n| n.0 * n.1).unwrap();
    let successors = |node: &Node| -> Vec<(Node, Cost)> {
        let mut successors = vec![];
        let right = node.0.checked_add(1).unwrap();
        let down = node.1.checked_add(1).unwrap();
        let mut add_successor = |next| {
            successors.push((next, map.0.get(&next).unwrap().0));
        };
        // left
        if let Some(x) = node.0.checked_sub(1) {
            add_successor((x, node.1));
        }
        // up
        if let Some(y) = node.1.checked_sub(1) {
            add_successor((node.0, y))
        }
        // right
        if map.0.contains_key(&(right, node.1)) {
            add_successor((right, node.1))
        }
        // down
        if map.0.contains_key(&(node.0, down)) {
            add_successor((node.0, down))
        }
        log::trace!("node: {:?}, successors: {:?}", node, successors);
        successors
    };
    let heuristic = |node: &Node| -> Cost {
        // get manhattan distance, to improve on dijkstra, but still get optimum
        log::trace!(
            "node: {:?}, heuristic: {:?}",
            node,
            (end.0 - node.0) + (end.1 - node.1)
        );
        (end.0 - node.0) + (end.1 - node.1)
    };
    let success = |node: &Node| -> bool {
        // get manhattan distance
        log::trace!("node: {:?}, success: {:?}", node, node == end);
        node == end
    };
    let (mut solutions, cost) = astar_bag(&start, successors, heuristic, success).unwrap();
    log::debug!("solution cost {}", cost);
    log::trace!("solution: {:?}", solutions.next());
    cost
}

pub fn part1(map: &Map) -> PartOutput<usize> {
    PartOutput { answer: path(map) }
}

pub fn part2(map: &Map) -> PartOutput<usize> {
    let mut map = Map(map.0.clone());
    map.full_map();
    PartOutput { answer: path(&map) }
}

pub const DAY: Day<Map, usize> = Day {
    title: "Chiton",
    display: (
        "The lowest total risk of any path is {answer}",
        "The lowest total risk of any path is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day15.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_part1() {
        let map = parse(DAY.example).unwrap();
        let result = path(&map);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_example_part2() {
        let mut map = parse(DAY.example).unwrap();
        assert_eq!(map.0.len(), 100);
        map.full_map();
        assert_eq!(map.0.len(), 2500);
        assert_eq!(map.0.get(&(49, 49)).unwrap().0, 9);
        assert_eq!(map.0.get(&(0, 49)).unwrap().0, 6);
        assert_eq!(map.0.get(&(1, 48)).unwrap().0, 6);
        let result = path(&map);
        assert_eq!(result, 315);
    }

    #[test]
    fn test_main() {
        let map = parse(&get_input(15)).unwrap();
        assert_eq!(part1(&map).answer.to_string(), "613");
        assert_eq!(part2(&map).answer.to_string(), "2899");
    }
}
