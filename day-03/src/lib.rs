use std::{collections::BTreeSet, error::Error, str::FromStr};

use advent_utils::{Part, Solver};

#[derive(Debug)]
pub struct Solution {
    packings: Vec<Vec<u8>>,
}

/// Get priority for byte (ASCII char), following rules:
///
/// * Lowercase item types a through z have priorities `1` through `26`.
/// * Uppercase item types A through Z have priorities `27` through `52`.
///
/// ```
/// use day_03::priority;
///
/// assert_eq!(priority(b'a'), 1);
/// assert_eq!(priority(b'c'), 3);
/// assert_eq!(priority(b'z'), 26);
/// assert_eq!(priority(b'Z'), 52);
/// ```
pub fn priority(c: u8) -> u8 {
    if c >= b'a' {
        c - b'a' + 1
    } else {
        c - b'A' + 1 + 26
    }
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let packings = s
            .lines()
            .map(|l| l.bytes().map(priority).collect())
            .collect();

        Ok(Self { packings })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let sum_errors = self
                    .packings
                    .iter()
                    .map(|packing| {
                        let lower_types = BTreeSet::from_iter(&packing[0..packing.len() / 2]);
                        let upper_types = BTreeSet::from_iter(&packing[packing.len() / 2..]);

                        **lower_types
                            .intersection(&upper_types)
                            .next()
                            .expect("no types are repeating") as u32
                    })
                    .sum::<u32>();

                format!("priorities sum for the mistaken types: {}", sum_errors)
            }
            Part::Two => {
                let sum_badges = self
                    .packings
                    .chunks_exact(3)
                    .map(|chunk| {
                        let set_a = BTreeSet::from_iter(&chunk[0]);
                        let set_b = BTreeSet::from_iter(&chunk[1]);
                        let set_c = BTreeSet::from_iter(&chunk[2]);

                        let common_ab: BTreeSet<_> = set_a.intersection(&set_b).copied().collect();

                        let common_abc = common_ab.intersection(&set_c).collect::<Vec<_>>();

                        assert_eq!(common_abc.len(), 1, "wrong number of common items in group");

                        (**common_abc[0]) as u32
                    })
                    .sum::<u32>();

                format!("priorities sum for the group names: {}", sum_badges)
            }
        }
    }

    fn day_number() -> u32 {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let solution: Solution = indoc::indoc!(
            "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"
        )
        .parse()
        .unwrap();

        assert_eq!(
            solution.solve(Part::One),
            "priorities sum for the mistaken types: 157"
        );
        assert_eq!(
            solution.solve(Part::Two),
            "priorities sum for the group names: 70"
        );
    }
}
