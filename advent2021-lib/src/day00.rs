use itertools::Itertools;

use crate::{Day, Parts};

pub fn calc(expenses: Vec<usize>, combinations: usize) -> Vec<Vec<usize>> {
    expenses
        .into_iter()
        .combinations(combinations)
        .filter(|v| v.iter().sum::<usize>() == 2020)
        .collect()
}

pub fn get_data(input: String) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().expect("bad input"))
        .collect()
}

pub fn main(input: String) -> Day {
    let expenses = get_data(input);
    match (2..=3)
        .into_iter()
        .map(|n| {
            let mut matches = calc(expenses.to_vec(), n);
            assert_eq!(matches.len(), 1);
            let values = matches.first_mut().unwrap();
            values.sort_unstable();
            let answer = values.iter().copied().product::<usize>();
            (
                answer,
                format!(
                    "{} = {}",
                    answer,
                    values.iter().copied().map(|v| v.to_string()).join(" × "),
                ),
            )
        })
        .collect::<Vec<(usize, String)>>()
        .drain(..)
        .as_slice()
    {
        [one, two] => Day {
            answers: Parts(one.0.to_string(), two.0.to_string()),
            display: Parts(one.1.to_string(), two.1.to_string()),
            ..Default::default()
        },
        _ => panic!("Unexpected parts"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let day = main();
        assert_eq!(day.answers.0, "158916");
        assert_eq!(day.answers.1, "165795564");
    }
}
