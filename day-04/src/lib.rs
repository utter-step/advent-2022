use std::{error::Error, ops::RangeInclusive, str::FromStr};

use advent_utils::{Part, Solver};
use color_eyre::eyre::Result;

#[derive(Debug)]
pub struct Solution {
    tasks: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>,
}

fn parse_range(s: &str) -> Result<RangeInclusive<u32>> {
    let (left, right) = s.split_once('-').expect("no delimiter in string");

    Ok(left.parse()?..=right.parse()?)
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tasks = s
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(',').unwrap();

                (parse_range(left).unwrap(), parse_range(right).unwrap())
            })
            .collect();

        Ok(Self { tasks })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let n_inclusive = self
                    .tasks
                    .iter()
                    .filter(|(left, right)| {
                        (left.contains(right.start()) && left.contains(right.end()))
                            || (right.contains(left.start()) && right.contains(left.end()))
                    })
                    .count();

                format!("{n_inclusive} ranges fully include one another")
            }
            Part::Two => {
                let n_overlaps = self
                    .tasks
                    .iter()
                    .filter(|(left, right)| {
                        left.contains(right.start())
                            || left.contains(right.end())
                            || right.contains(left.start())
                            || right.contains(left.end())
                    })
                    .count();

                format!("{n_overlaps} ranges intersect")
            }
        }
    }

    fn day_number() -> u32 {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let solution: Solution = indoc::indoc! {
            "2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8"
        }
        .parse()
        .expect("failed to parse Solution");

        assert_eq!(
            solution.solve(Part::One),
            "2 ranges fully include one another"
        );
        assert_eq!(solution.solve(Part::Two), "4 ranges intersect");
    }
}
