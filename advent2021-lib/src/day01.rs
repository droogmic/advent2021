use crate::{Day, DayCalc, PartOutput};

pub fn get_data(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().expect("bad input"))
        .collect()
}

enum MeasurementChange {
    Flat,
    Increasing,
    Decreasing,
}

pub fn depths_increasing(sonar_depths: &[usize], size: usize) -> usize {
    sonar_depths
        .windows(size)
        .map(|window| match window {
            w if w.first() < w.last() => MeasurementChange::Increasing,
            w if w.first() > w.last() => MeasurementChange::Decreasing,
            w if w.first() == w.last() => MeasurementChange::Flat,
            _ => unreachable!(),
        })
        .filter(|change| matches!(change, MeasurementChange::Increasing))
        .count()
}

pub fn part1(sonar_depths: &[usize]) -> PartOutput<usize> {
    let increasing = depths_increasing(sonar_depths, 2);
    PartOutput { answer: increasing }
}

pub fn part2(sonar_depths: &[usize]) -> PartOutput<usize> {
    let threes_increasing = depths_increasing(sonar_depths, 4);
    PartOutput {
        answer: threes_increasing,
    }
}

pub const DAY: Day<Vec<usize>, [usize], usize> = Day {
    title: "Sonar Sweep",
    display: (
        "There are {answer} measurements larger than the previous measurement",
        "There are {answer} sums larger than the previous sum",
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
        let result = depths_increasing(
            &get_data("199\n200\n208\n210\n200\n207\n240\n269\n260\n263".into()),
            2,
        );
        assert_eq!(result, 7);
    }

    #[test]
    fn test_example_part2() {
        let result = depths_increasing(
            &get_data("199\n200\n208\n210\n200\n207\n240\n269\n260\n263".into()),
            4,
        );
        assert_eq!(result, 5);
    }

    #[test]
    fn test_main() {
        let input = get_data(&get_input(1));
        assert_eq!(part1(&input).answer.to_string(), "1393");
        assert_eq!(part2(&input).answer.to_string(), "1359");
    }
}
