use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

#[derive(Debug)]
pub struct Solution {
    calories_data: Vec<i64>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut calories_data: Vec<_> = s
            .split("\n\n")
            .map(|one_elf_data| {
                one_elf_data
                    .split('\n')
                    .filter_map(|calories| calories.parse::<i64>().ok())
                    .sum()
            })
            .collect();
        calories_data.sort_unstable_by(|a: &i64, b: &i64| b.cmp(a));

        Ok(Self { calories_data })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!(
                "elf with most reserves has total of {} calories",
                self.calories_data[0]
            ),
            Part::Two => format!(
                "top 3 elves have total of {} calories",
                self.calories_data[..3].iter().sum::<i64>()
            ),
        }
    }

    fn day_number() -> u32 {
        1
    }

    fn implemented_parts() -> Vec<Part> {
        vec![Part::One, Part::Two]
    }
}
