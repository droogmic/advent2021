use crate::{Day, Parts};

use itertools::{izip, zip};

pub fn get_data(input: String) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().expect("bad input"))
        .collect()
}

enum MeasurementChange {
    FLT,
    INC,
    DEC,
}

pub fn depths_increasing(sonar_depths: &[usize]) -> usize {
    zip(sonar_depths, &sonar_depths[1..])
        .map(|(first, second)| {
            if second > first {
                MeasurementChange::INC
            } else if first > second {
                MeasurementChange::DEC
            } else {
                MeasurementChange::FLT
            }
        })
        .filter(|change| matches!(change, MeasurementChange::INC))
        .count()
}

pub fn depths_increasing_threes(sonar_depths: &[usize]) -> usize {
    zip(
        izip!(sonar_depths, &sonar_depths[1..], &sonar_depths[2..]),
        izip!(&sonar_depths[1..], &sonar_depths[2..], &sonar_depths[3..]),
    )
    .map(|((a, b, c), (x, y, z))| {
        let first = a + b + c;
        let second = x + y + z;
        if second > first {
            MeasurementChange::INC
        } else if first > second {
            MeasurementChange::DEC
        } else {
            MeasurementChange::FLT
        }
    })
    .filter(|change| matches!(change, MeasurementChange::INC))
    .count()
}

pub fn main(input: String) -> Day {
    let sonar_depths = get_data(input);
    let increasing = depths_increasing(&sonar_depths);
    let threes_increasing = depths_increasing_threes(&sonar_depths);

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
        let result = depths_increasing(&get_data(
            "199\n200\n208\n210\n200\n207\n240\n269\n260\n263".into(),
        ));
        assert_eq!(result, 7);
    }

    #[test]
    fn test_example_part2() {
        let result = depths_increasing_threes(&get_data(
            "199\n200\n208\n210\n200\n207\n240\n269\n260\n263".into(),
        ));
        assert_eq!(result, 5);
    }

    #[test]
    fn test_main() {
        let day = main(get_string(1));
        assert_eq!(day.answers.0, "1393");
        assert_eq!(day.answers.1, "1359");
    }
}
