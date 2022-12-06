use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

/// Rock-paper-scisors game move.
///
/// Each move has a draw with itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Move {
    /// Beats scisors, loses to paper
    Rock,
    /// Beats rock, loses to scisors
    Paper,
    /// Beats paper, loses to rock
    Scisors,
}

/// Rock-paper-scisors game outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Outcome {
    Lost,
    Draw,
    Won,
}

impl Outcome {
    pub(crate) fn points(self) -> usize {
        match self {
            Self::Lost => 0,
            Self::Draw => 3,
            Self::Won => 6,
        }
    }

    pub(crate) fn infere_move(self, other: Move) -> Move {
        const MOVE_MAP: [Move; 5] = [
            Move::Scisors,
            Move::Rock,
            Move::Paper,
            Move::Scisors,
            Move::Rock,
        ];

        match self {
            Outcome::Draw => other,
            Outcome::Won => MOVE_MAP[other.points() + 1],
            Outcome::Lost => MOVE_MAP[other.points() - 1],
        }
    }
}

impl Move {
    pub(crate) fn points(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scisors => 3,
        }
    }

    pub(crate) fn calculate_outcome(self, other: Move) -> Outcome {
        let self_num = self.points();
        let other_num = other.points();

        let winner = match self_num.abs_diff(other_num) {
            0 => return Outcome::Draw,
            1 => self.max(other),
            2 => self.min(other),
            _ => unreachable!(),
        };

        if self == winner {
            Outcome::Won
        } else {
            Outcome::Lost
        }
    }
}

#[derive(Debug)]
pub struct Solution {
    guide: Vec<(Move, char)>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let guide = s
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(' ').expect("failed to split guide entry");
                assert!(left.len() == 1 && right.len() == 1);

                let left = match left.chars().next().unwrap() {
                    'A' => Move::Rock,
                    'B' => Move::Paper,
                    'C' => Move::Scisors,
                    _ => unreachable!(),
                };

                (left, right.chars().next().unwrap())
            })
            .collect();

        Ok(Self { guide })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        2
    }

    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let score = self
                    .guide
                    .iter()
                    .map(|&(other_move, my_move)| {
                        let my_move = match my_move {
                            'X' => Move::Rock,
                            'Y' => Move::Paper,
                            'Z' => Move::Scisors,
                            _ => unreachable!(),
                        };

                        my_move.points() + my_move.calculate_outcome(other_move).points()
                    })
                    .sum::<usize>();

                format!("you will end up with total score of {}", score)
            }
            Part::Two => {
                let score = self
                    .guide
                    .iter()
                    .map(|&(other_move, outcome)| {
                        let outcome = match outcome {
                            'X' => Outcome::Lost,
                            'Y' => Outcome::Draw,
                            'Z' => Outcome::Won,
                            _ => unreachable!(),
                        };

                        outcome.points() + outcome.infere_move(other_move).points()
                    })
                    .sum::<usize>();

                format!("actually, your score will be {}", score)
            }
        }
    }

    fn implemented_parts() -> Vec<Part> {
        vec![Part::One, Part::Two]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let solution: Solution = "A Y\nB X\nC Z\n".parse().unwrap();

        assert_eq!(
            &solution.solve(Part::One),
            "you will end up with total score of 15"
        );
        assert_eq!(
            &solution.solve(Part::Two),
            "actually, your score will be 12"
        );
    }

    #[test]
    fn test_outcome() {
        assert_eq!(Move::Rock.calculate_outcome(Move::Rock), Outcome::Draw);
        assert_eq!(Move::Rock.calculate_outcome(Move::Paper), Outcome::Lost);
        assert_eq!(Move::Rock.calculate_outcome(Move::Scisors), Outcome::Won);

        assert_eq!(Move::Paper.calculate_outcome(Move::Rock), Outcome::Won);
        assert_eq!(Move::Paper.calculate_outcome(Move::Paper), Outcome::Draw);
        assert_eq!(Move::Paper.calculate_outcome(Move::Scisors), Outcome::Lost);

        assert_eq!(Move::Scisors.calculate_outcome(Move::Rock), Outcome::Lost);
        assert_eq!(Move::Scisors.calculate_outcome(Move::Paper), Outcome::Won);
        assert_eq!(
            Move::Scisors.calculate_outcome(Move::Scisors),
            Outcome::Draw
        );
    }

    #[test]
    fn test_move() {
        for a_move in [Move::Rock, Move::Paper, Move::Scisors] {
            for b_move in [Move::Rock, Move::Paper, Move::Scisors] {
                assert_eq!(
                    a_move.calculate_outcome(b_move).infere_move(b_move),
                    a_move,
                    "a: {a_move:?}, b: {b_move:?}",
                );
            }
        }
    }
}
