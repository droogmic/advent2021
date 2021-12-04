use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

pub struct SonarDepths(Vec<usize>);

pub fn get_data(input: &str) -> ParseResult<SonarDepths> {
    Ok(SonarDepths(
        input
            .lines()
            .map(|line| line.parse().map_err(|_| ParseError {}))
            .collect::<ParseResult<_>>()?,
    ))
}

enum MeasurementChange {
    Flat,
    Increasing,
    Decreasing,
}

pub fn depths_increasing(sonar_depths: &SonarDepths, size: usize) -> usize {
    sonar_depths
        .0
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

pub fn part1(sonar_depths: &SonarDepths) -> PartOutput<usize> {
    let increasing = depths_increasing(sonar_depths, 2);
    PartOutput { answer: increasing }
}

pub fn part2(sonar_depths: &SonarDepths) -> PartOutput<usize> {
    let threes_increasing = depths_increasing(sonar_depths, 4);
    PartOutput {
        answer: threes_increasing,
    }
}

pub const DAY: Day<SonarDepths, usize> = Day {
    title: "Sonar Sweep",
    display: (
        "There are {answer} measurements larger than the previous measurement",
        "There are {answer} sums larger than the previous sum",
    ),
    calc: DayCalc {
        parse: get_data,
        part1,
        part2,
    },
    example: "199\n200\n208\n210\n200\n207\n240\n269\n260\n263",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;

    #[test]
    fn test_example_part1() {
        let result = depths_increasing(&get_data(DAY.example).unwrap(), 2);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_example_part2() {
        let result = depths_increasing(&get_data(DAY.example).unwrap(), 4);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_main() {
        let input = get_data(&get_input(1)).unwrap();
        assert_eq!(part1(&input).answer.to_string(), "1393");
        assert_eq!(part2(&input).answer.to_string(), "1359");
    }
}
