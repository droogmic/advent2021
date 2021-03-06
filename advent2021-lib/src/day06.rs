use ndarray::prelude::*;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone, Default, Debug)]
pub struct LanternfishState(pub Vec<usize>);

pub fn parse(input: &str) -> ParseResult<LanternfishState> {
    Ok(LanternfishState(
        input
            .trim()
            .split(',')
            .map(|val| val.parse().map_err(ParseError::Int))
            .collect::<ParseResult<_>>()?,
    ))
}

/// One day of lanternfish.
///
/// ```
/// let state = advent2021_lib::day06::fish_step(vec![2,3,2,0,1]);
/// assert_eq!(state, vec![1,2,1,6,8,0]);
/// ```
pub fn fish_step(state: Vec<usize>) -> Vec<usize> {
    fn get_fish_step_state(fish: usize) -> Vec<usize> {
        match fish {
            1..=usize::MAX => vec![fish - 1],
            0 => vec![6, 8],
            _ => unreachable!(),
        }
    }
    state.into_iter().flat_map(get_fish_step_state).collect()
}
/// Naïve solution.
pub fn fish_steps(mut state: Vec<usize>, n: usize) -> Vec<usize> {
    for _ in 0..n {
        state = fish_step(state);
    }
    state
}

/// Counter array solution.
///
/// ```
/// let count = advent2021_lib::day06::fish_count_array(&vec![3,4,3,1,2], 18);
/// assert_eq!(count, 26);
/// ```
pub fn fish_count_array(state: &[usize], n: usize) -> usize {
    fn fish_count(counts: [usize; 9]) -> [usize; 9] {
        [
            counts[1],
            counts[2],
            counts[3],
            counts[4],
            counts[5],
            counts[6],
            counts[7] + counts[0],
            counts[8],
            counts[0],
        ]
    }
    let mut counts = [0; 9];
    for (num, count) in counts.iter_mut().enumerate() {
        *count = state.iter().filter(|&&fish| fish == num).count();
    }
    for _ in 0..n {
        counts = fish_count(counts);
    }
    counts.into_iter().sum()
}

/// Matrix solution
///
/// ```
/// let count = advent2021_lib::day06::fish_count_ndarray(&vec![3,4,3,1,2], 18);
/// assert_eq!(count, 26);
/// ```
pub fn fish_count_ndarray(state: &[usize], n: usize) -> usize {
    let day = array![
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let days = (0..n).fold(Array::eye(9), |acc, _| acc.dot(&day));
    let mut counts = Array::zeros((9, 1));
    for num in 0..9 {
        counts[[num, 0]] = state.iter().filter(|&&fish| fish == num).count();
    }
    let counts = days.dot(&counts);
    counts.into_iter().sum()
}

pub fn part1(state: &LanternfishState) -> PartOutput<usize> {
    let count = fish_count_array(&state.0, 80);
    PartOutput { answer: count }
}

pub fn part2(state: &LanternfishState) -> PartOutput<usize> {
    let count = fish_count_array(&state.0, 256);
    PartOutput { answer: count }
}

pub const DAY: Day<LanternfishState, usize> = Day {
    title: "Lanternfish",
    display: (
        "{answer} lanternfish after 80 days",
        "{answer} lanternfish after 256 days",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: "3,4,3,1,2",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_part1_vec() {
        let state = parse(DAY.example).unwrap();
        let result = fish_steps(state.clone().0, 80);
        assert_eq!(result.len(), 5934);
    }

    #[test]
    fn test_example_part1_counts() {
        let state = parse(DAY.example).unwrap();
        let result = fish_count_array(&state.0, 80);
        assert_eq!(result, 5934);
    }

    #[test]
    fn test_example_part1_ndarray() {
        let state = parse(DAY.example).unwrap();
        let result = fish_count_ndarray(&state.0, 80);
        assert_eq!(result, 5934);
    }

    #[test]
    fn test_example_part2_counts() {
        let state = parse(DAY.example).unwrap();
        let result = fish_count_array(&state.0, 256);
        assert_eq!(result, 26984457539);
    }

    #[test]
    fn test_main() {
        let state = parse(&get_input(6)).unwrap();
        assert_eq!(part1(&state).answer.to_string(), "386755");
        assert_eq!(part2(&state).answer.to_string(), "1732731810807");
    }
}
