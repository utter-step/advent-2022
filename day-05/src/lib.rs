use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};
use color_eyre::{eyre::eyre, Report, Result};
use wyz::BidiIterator;

#[derive(Debug)]
struct Movement {
    from: usize,
    to: usize,
    n_to_move: usize,
}

impl FromStr for Movement {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n_to_move, rest) = s
            .strip_prefix("move ")
            .and_then(|rest| rest.split_once(' '))
            .ok_or_else(|| eyre!("unknown movement format: {s}"))?;

        let (from, rest) = rest
            .strip_prefix("from ")
            .and_then(|rest| rest.split_once(' '))
            .ok_or_else(|| eyre!("unknown movement format: {s}"))?;

        let to = rest
            .strip_prefix("to ")
            .ok_or_else(|| eyre!("unknown movement format: {s}"))?;

        Ok(Self {
            from: from.parse()?,
            to: to.parse()?,
            n_to_move: n_to_move.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Solution {
    stacks: Vec<Vec<char>>,

    movements: Vec<Movement>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stacks, movements) = s
            .split_once("\n\n")
            .ok_or_else(|| eyre!("cannot split initial data"))?;
        let mut stack_lines = stacks.lines().rev();

        let n_stacks = stack_lines
            .next()
            .ok_or_else(|| eyre!("no data in stacks"))?
            .split_ascii_whitespace()
            .count();

        if n_stacks > 9 {
            return Err(eyre!(
                "current solution only nows how to deal with no more than 9 stacks :("
            ))?;
        }

        let mut stacks = Vec::with_capacity(n_stacks);
        stacks.resize(n_stacks, Vec::with_capacity(stack_lines.size_hint().0));
        for mut line in stack_lines {
            /// `[A]` – that's how crates are specified
            const CRATE_WIDTH: usize = 3;

            let mut idx = 0;
            while !line.is_empty() {
                let (crate_, rest) = if line.len() > 4 {
                    line.split_at(CRATE_WIDTH + 1)
                } else {
                    (line, "")
                };

                if !crate_.trim().is_empty() {
                    stacks[idx].push(
                        crate_
                            .chars()
                            .nth(1)
                            .ok_or_else(|| eyre!("crate with unknown format: {crate_}"))?,
                    );
                }

                idx += 1;
                line = rest;
            }
        }

        let movements = movements
            .lines()
            .map(Movement::from_str)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { stacks, movements })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        let mut stacks = self.stacks.clone();

        for &Movement {
            from,
            to,
            n_to_move,
        } in &self.movements
        {
            let from_len = stacks[from - 1].len();
            let mut moved = {
                stacks[from - 1]
                    .drain(from_len - n_to_move..)
                    // in part one mover reverses the crates while moving them, in part two – not
                    .bidi(matches!(part, Part::One))
                    .collect()
            };

            stacks[to - 1].append(&mut moved);
        }

        let final_: String = stacks
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect();

        format!("top crates are: `{final_}`")
    }

    fn day_number() -> u32 {
        5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let solution: Solution = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
            .parse()
            .expect("failed to parse solution");

        assert_eq!(solution.solve(Part::One), "top crates are: `CMZ`");
        assert_eq!(solution.solve(Part::Two), "top crates are: `MCD`");
    }
}
