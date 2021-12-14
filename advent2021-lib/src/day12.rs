use std::collections::HashMap;
use std::collections::HashSet;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone, Debug)]
pub struct Routes(HashMap<String, Vec<String>>);

#[derive(Clone, Debug)]
struct Path<'a> {
    caves: Vec<&'a str>,
    end: bool,
    small_caves: HashSet<&'a str>,
    small_cave_twice: bool,
}

impl<'a> Path<'a> {
    fn new(caves: Vec<&'a str>) -> Self {
        Self {
            caves,
            end: false,
            small_caves: HashSet::new(),
            small_cave_twice: false,
        }
    }

    /// Add next path segment
    /// returns None if a small cave is visited too often
    /// returns a path if not
    fn add_next(&self, next: &'a str, allow_small_cave_twice: bool) -> Option<Self> {
        let Self {
            caves,
            end,
            small_caves,
            small_cave_twice,
        } = self;
        log::trace!("> add_next {:?} onto {:?}", next, self,);
        assert!(!end);
        let another_small_cave = small_caves.contains(next);
        if another_small_cave && (!allow_small_cave_twice || *small_cave_twice) {
            log::trace!("< add_next discarding");
            return None;
        }
        let small_caves = {
            let mut small_caves = small_caves.clone();
            let is_small_cave = next == next.to_lowercase() && next != "end";
            if is_small_cave {
                let dup = !small_caves.insert(next);
                assert_eq!(dup, another_small_cave)
            }
            small_caves
        };
        let mut extended_caves = caves.to_vec();
        extended_caves.push(next);
        log::trace!(
            "< add_next extending {:?}, {:?}, {:?}",
            extended_caves,
            small_caves,
            another_small_cave
        );
        if another_small_cave && *small_cave_twice {
            assert_eq!(caves.iter().filter(|cave| cave == &&next).count(), 1);
        }
        Some(Self {
            caves: extended_caves,
            end: next == "end",
            small_caves,
            small_cave_twice: *small_cave_twice || another_small_cave,
        })
    }
}

impl Routes {
    fn path_steps<'a>(&'a self, path: Path<'a>, allow_small_cave_twice: bool) -> Vec<Path<'a>> {
        if path.end {
            log::trace!("path {:?} is at end", path.caves);
            return vec![path];
        }
        let last = path.caves.last().unwrap();
        log::trace!("path {:?}", path);
        match self.0.get(last.to_owned()) {
            None => {
                if last == &"end" {
                    panic!("Path State Inconsistent");
                } else {
                    panic!("Dead End Not Possible");
                }
            }
            Some(options) => options
                .iter()
                .filter_map(|next| path.add_next(next, allow_small_cave_twice))
                .map(|path| self.path_steps(path, allow_small_cave_twice))
                .flatten()
                .collect(),
        }
    }

    fn count_paths(&self, allow_small_cave_twice: bool) -> usize {
        let path = Path::new(vec!["start"]);
        let paths = self.path_steps(path, allow_small_cave_twice);
        log::debug!("paths: {:?}", paths);
        paths.len()
    }
}

pub fn parse(input: &str) -> ParseResult<Routes> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split('-');
        let left = parts.next().ok_or(ParseError::Empty)?;
        let right = parts.next().ok_or(ParseError::Empty)?;
        if right != "start" && left != "end" {
            let dest = map.entry(left.to_owned()).or_insert_with(Vec::new);
            dest.push(right.to_owned());
        }
        if left != "start" && right != "end" {
            let dest = map.entry(right.to_owned()).or_insert_with(Vec::new);
            dest.push(left.to_owned());
        }
    }
    log::debug!("map: {:?}", map);
    Ok(Routes(map))
}

pub fn part1(routes: &Routes) -> PartOutput<usize> {
    PartOutput {
        answer: routes.count_paths(false),
    }
}

pub fn part2(routes: &Routes) -> PartOutput<usize> {
    PartOutput {
        answer: routes.count_paths(true),
    }
}

pub const DAY: Day<Routes, usize> = Day {
    title: "TITLE",
    display: ("There are {answer} paths", "There are {answer} paths"),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day12.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example1_part1() {
        let routes = parse("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end").unwrap();
        assert_eq!(routes.count_paths(false), 10);
    }

    #[test]
    fn test_example2_part1() {
        let routes = parse(
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc",
        )
        .unwrap();
        assert_eq!(routes.count_paths(false), 19);
    }

    #[test]
    fn test_example3_part1() {
        let routes = parse(DAY.example).unwrap();
        assert_eq!(routes.count_paths(false), 226);
    }

    #[test]
    fn test_example1_part2() {
        let routes = parse("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end").unwrap();
        assert_eq!(routes.count_paths(true), 36);
    }

    #[test]
    fn test_example2_part2() {
        let routes = parse(
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc",
        )
        .unwrap();
        assert_eq!(routes.count_paths(true), 103);
    }

    #[test]
    fn test_example3_part2() {
        let routes = parse(DAY.example).unwrap();
        assert_eq!(routes.count_paths(true), 3509);
    }

    #[test]
    fn test_main() {
        let routes = parse(&get_input(12)).unwrap();
        assert_eq!(part1(&routes).answer.to_string(), "5958");
        assert_eq!(part2(&routes).answer.to_string(), "150426");
    }
}
