use std::collections::HashSet;
use std::fmt;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

pub struct Numbers(pub Vec<usize>);

impl fmt::Binary for Numbers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // extract the value using tuple idexing
        // and create reference to 'vec'
        let vec = &self.0;

        // @count -> the index of the value,
        // @n     -> the value
        for (count, n) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, " ")?;
            }

            write!(f, "{:b}", n)?;
        }

        Ok(())
    }
}

pub struct Report {
    pub numbers: Numbers,
    pub width: usize,
}

pub fn get_report(input: &str) -> ParseResult<Report> {
    Ok(Report {
        numbers: Numbers(
            input
                .lines()
                .map(|line| usize::from_str_radix(line, 2).map_err(ParseError::Int))
                .collect::<ParseResult<_>>()?,
        ),
        width: input.lines().next().unwrap().len(),
    })
}

/// # Get average bitwise
///
/// ```
/// let report = advent2021_lib::day03::Report {
///     numbers: advent2021_lib::day03::Numbers(vec![
///         0b1111_1100,
///         0b1111_0000,
///         0b1111_0000,
///         0b1100_0000,
///     ]),
///     width: 8,
/// };
/// let avg = advent2021_lib::day03::get_bitwise_avg(&report);
/// assert_eq!(avg, 0b1111_0000);
/// ```
///
pub fn get_bitwise_avg(report: &Report) -> usize {
    let numbers = &report.numbers.0;
    let width = report.width;
    let low_upper_bound_inclusive = numbers
        .len()
        .checked_sub(1)
        .unwrap()
        .checked_div(2)
        .unwrap();
    let high_lower_bound_inclusive = numbers
        .len()
        .checked_add(2)
        .unwrap()
        .checked_div(2)
        .unwrap();
    (0..width)
        .map(|mask_len| {
            let row_sum = numbers
                .iter()
                .fold(0, |acc, val| acc + ((val >> mask_len) & 1));
            match row_sum {
                s if s >= high_lower_bound_inclusive => 1,
                s if s <= low_upper_bound_inclusive => 0,
                _ => panic!("unexpected sum"),
            }
        })
        .fold((0, 0), |(shift, acc), val| {
            (shift + 1, acc + (val << shift))
        })
        .1
}

///
/// ```
/// assert_eq!(9, advent2021_lib::day03::invert(22, 5));
/// ```
///
pub fn invert(number: usize, len: usize) -> usize {
    ((1 << len) - 1) & !number
}

///
/// ```
/// assert_eq!(9, advent2021_lib::day03::gamma_rate_to_epsilon_rate(22, 5));
/// ```
///
pub fn gamma_rate_to_epsilon_rate(gamma_rate: usize, len: usize) -> usize {
    invert(gamma_rate, len)
}

///
/// ```
/// assert_eq!(198, advent2021_lib::day03::gamma_rate_to_power_rate(22, 5));
/// ```
///
pub fn gamma_rate_to_power_rate(gamma_rate: usize, len: usize) -> usize {
    gamma_rate * gamma_rate_to_epsilon_rate(gamma_rate, len)
}

pub enum LifeSupport {
    Oxygen,
    Co2,
}

///
/// Get rating from largest to smallest bit search, by default it gets the oxygen scrubber rating
///
/// ```
/// let report = advent2021_lib::day03::Report {
///     numbers: advent2021_lib::day03::Numbers(vec![
///         0b1111_1100,
///         0b1111_0000,
///         0b1111_0100,
///         0b1100_0000,
///     ]),
///     width: 8,
/// };
/// let avg = advent2021_lib::day03::get_rating(&report, advent2021_lib::day03::LifeSupport::Oxygen);
/// assert_eq!(avg, 0b1111_0100);
/// ```
///
pub fn get_rating(report: &Report, life_support: LifeSupport) -> usize {
    let mut report_rows: HashSet<usize> = report.numbers.0.iter().cloned().collect();
    let mut mask_offset = report.width;
    log::info!(
        "rating  {:b}",
        Numbers(report_rows.iter().cloned().collect())
    );
    while report_rows.len() > 1 {
        mask_offset -= 1;
        let low_upper_bound_inclusive = report_rows
            .len()
            .checked_sub(1)
            .unwrap()
            .checked_div(2)
            .unwrap();
        let high_lower_bound_inclusive = report_rows
            .len()
            .checked_add(1)
            .unwrap()
            .checked_div(2)
            .unwrap();
        let bit_sum = report_rows
            .iter()
            .fold(0, |acc, val| acc + ((val >> mask_offset) & 1));
        let bit_to_keep = match bit_sum {
            s if s >= high_lower_bound_inclusive => 1,
            s if s <= low_upper_bound_inclusive => 0,
            _ => panic!("unexpected sum"),
        };
        let bit_to_keep = match life_support {
            LifeSupport::Oxygen => bit_to_keep,
            LifeSupport::Co2 => 1 - bit_to_keep,
        };
        report_rows.retain(|number| ((number >> mask_offset) & 1) == bit_to_keep);
        log::info!(
            "        {:b}",
            Numbers(report_rows.iter().cloned().collect())
        );
    }
    let rating = report_rows.drain().next().unwrap();
    rating
}

pub fn get_oxygen_rating(report: &Report) -> usize {
    get_rating(report, LifeSupport::Oxygen)
}

pub fn get_co2_rating(report: &Report) -> usize {
    get_rating(report, LifeSupport::Co2)
}

pub fn part1(report: &Report) -> PartOutput<usize> {
    let gamma_rate = get_bitwise_avg(report);
    let power_consumption = gamma_rate_to_power_rate(gamma_rate, report.width);
    PartOutput {
        answer: power_consumption,
    }
}

pub fn part2(report: &Report) -> PartOutput<usize> {
    let oxygen_rating = get_oxygen_rating(report);
    let co2_rating = get_co2_rating(report);
    let life_support_rating = oxygen_rating * co2_rating;
    PartOutput {
        answer: life_support_rating,
    }
}

pub const DAY: Day<Report, usize> = Day {
    title: "Binary Diagnostic",
    display: (
        "The power consumption of the submarine is {answer}",
        "The life support rating is {answer}",
    ),
    calc: DayCalc {
        parse: get_report,
        part1,
        part2,
    },
    example: "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_part1() {
        let report = get_report(DAY.example).unwrap();
        let result = get_bitwise_avg(&report);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_example_part2() {
        let report = get_report(DAY.example).unwrap();
        let oxygen_result = get_oxygen_rating(&report);
        assert_eq!(oxygen_result, 23);
        let co2_result = get_co2_rating(&report);
        assert_eq!(co2_result, 10);
    }

    #[test]
    fn test_main() {
        let report = get_report(&get_input(3)).unwrap();
        assert_eq!(part1(&report).answer.to_string(), "2972336");
        assert_eq!(part2(&report).answer.to_string(), "3368358");
    }
}
