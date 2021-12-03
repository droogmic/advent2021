use std::collections::HashSet;
use std::fmt;

use crate::{Day, DayCalc, PartOutput};

struct Numbers(Vec<usize>);

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

struct Report {
    numbers: Numbers,
    width: usize,
}

pub fn get_binary_rows(input: &str) -> Report {
    Report {
        numbers: input
            .lines()
            .map(|line| usize::from_str_radix(line, 2).expect("invalid binary int"))
            .collect(),
        width: input.lines().next().unwrap().len(),
    }
}

/// # Get average bitwise
///
/// ```
/// let values = vec![
///     0b1111_1100,
///     0b1111_0000,
///     0b1111_0000,
///     0b1100_0000,
/// ];
/// let avg = advent2021_lib::day03::get_bitwise_avg(&values, 8);
/// assert_eq!(avg, 0b1111_0000);
/// ```
///
pub fn get_bitwise_avg(numbers: &Numbers, row_len: usize) -> usize {
    let numbers = numbers.0;
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
    (0..row_len)
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
/// let values = vec![
///     0b1111_1100,
///     0b1111_0000,
///     0b1111_0100,
///     0b1100_0000,
/// ];
/// let avg = advent2021_lib::day03::get_rating(&values, 8, advent2021_lib::day03::LifeSupport::Oxygen);
/// assert_eq!(avg, 0b1111_0100);
/// ```
///
pub fn get_rating(report_rows: &[usize], row_len: usize, life_support: LifeSupport) -> usize {
    let mut report_rows: HashSet<usize> = report_rows.iter().cloned().collect();
    let mut mask_offset = row_len;
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
        log::info!("{:b}", Numbers(report_rows.iter().cloned().collect()));
    }
    let rating = report_rows.drain().next().unwrap();
    rating
}

pub fn get_oxygen_rating(report_rows: &Numbers, row_len: usize) -> usize {
    get_rating(report_rows, row_len, LifeSupport::Oxygen)
}

pub fn get_co2_rating(report_rows: &Numbers, row_len: usize) -> usize {
    get_rating(report_rows, row_len, LifeSupport::Co2)
}

pub fn part1(report: &Report) -> PartOutput<usize> {
    let gamma_rate = get_bitwise_avg(report.numbers, report.width);
    let power_consumption = gamma_rate_to_power_rate(gamma_rate, row_len);
    PartOutput {
        answer: power_consumption,
    }
}

pub fn part2(report: &Report) -> PartOutput<usize> {
    let oxygen_rating = get_oxygen_rating(report.numbers, report.width);
    let co2_rating = get_co2_rating(&report_rows, row_len);
    let life_support_rating = oxygen_rating * co2_rating;
    PartOutput {
        answer: life_support_rating,
    }
}

pub const DAY: Day<Report, Report, usize> = Day {
    title: "Dive!",
    display: (
        "The horizontal position to final depth product is {answer}",
        "The horizontal position to final depth product is {answer}",
    ),
    calc: DayCalc {
        parse: get_binary_rows,
        part1: part1,
        part2: part2,
    },
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    const EXAMPLE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_example_part1() {
        let report = get_binary_rows(EXAMPLE);
        let result = get_bitwise_avg(&report.numbers, report.width);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_example_part2() {
        let report = get_binary_rows(EXAMPLE);
        let oxygen_result = get_oxygen_rating(&report.numbers, report.width);
        assert_eq!(oxygen_result, 23);
        let co2_result = get_co2_rating(&report.numbers, report.width);
        log::info!("TEST");
        assert_eq!(co2_result, 10);
    }

    #[test]
    fn test_main() {
        let day = main(get_input(3));
        assert_eq!(day.answers.0, "2972336");
        assert_eq!(day.answers.1, "3368358");
    }
}
