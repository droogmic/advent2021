use crate::{Day, Parts};

pub fn get_data(input: String) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().expect("bad input"))
        .collect()
}

enum MeasurementChange {
    Flt,
    Inc,
    Dec,
}

pub fn depths_increasing(sonar_depths: &[usize], size: usize) -> usize {
    sonar_depths
        .windows(size)
        .map(|window| match window {
            w if w.first() < w.last() => MeasurementChange::Inc,
            w if w.first() > w.last() => MeasurementChange::Dec,
            w if w.first() == w.last() => MeasurementChange::Flt,
            _ => unreachable!(),
        })
        .filter(|change| matches!(change, MeasurementChange::Inc))
        .count()
}

pub fn main(input: String) -> Day {
    let sonar_depths = get_data(input);
    let increasing = depths_increasing(&sonar_depths, 2);
    // Average of 3 is like a window of 4
    let threes_increasing = depths_increasing(&sonar_depths, 4);

    Day {
        answers: Parts(increasing.to_string(), threes_increasing.to_string()),
        display: Parts(
            format!(
                "There are {} measurements larger than the previous measurement",
                increasing
            ),
            format!(
                "There are {} sums larger than the previous sum",
                threes_increasing
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
        let day = main(get_string(1));
        assert_eq!(day.answers.0, "1393");
        assert_eq!(day.answers.1, "1359");
    }
}
