use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

#[derive(Debug)]
pub struct Solution {
    map: Vec<Vec<u8>>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            map: s
                .lines()
                .map(|line| line.bytes().map(|b| b - b'0').collect())
                .collect(),
        })
    }
}

fn check_if_visible_naive(map: &[Vec<u8>], x: usize, y: usize) -> bool {
    let len = map.len();

    if x == 0 || x == len - 1 || y == 0 || y == len - 1 {
        return true;
    }

    let current = map[y][x];

    (0..y).map(|y| map[y][x]).all(|height| height < current) // from top
    || (y + 1..len).map(|y| map[y][x]).all(|height| height < current) // from bottom
    || (0..x).map(|x| map[y][x]).all(|height| height < current) // from left
    || (x + 1..len).map(|x| map[y][x]).all(|height| height < current) // from right
}

fn compute_score_naive(map: &[Vec<u8>], x: usize, y: usize) -> u32 {
    let len = map.len();

    if x == 0 || x == len - 1 || y == 0 || y == len - 1 {
        return 0;
    }

    macro_rules! stop_if_zero {
        ($check: expr, $do: expr) => {
            if $check == 0 {
                return 0;
            } else {
                $do
            }
        };
    }

    let current = map[y][x];

    let nearest_top = (
        y - (0..y).rev().find(|&y| map[y][x] >= current).unwrap_or(0)
    ) as u32;
    let nearest_bottom = stop_if_zero!(
        nearest_top,
        (y + 1..len).find(|&y| map[y][x] >= current).unwrap_or(len - 1) - y
    ) as u32;
    let nearest_left = stop_if_zero!(
        nearest_bottom,
        x - (0..x).rev().find(|&x| map[y][x] >= current).unwrap_or(0)
    ) as u32;
    let nearest_right = stop_if_zero!(
        nearest_left,
        (x + 1..len).find(|&x| map[y][x] >= current).unwrap_or(len - 1) - x
    ) as u32;

    nearest_top * nearest_bottom * nearest_left * nearest_right
}

fn count_visible(map: &[Vec<u8>]) -> usize {
    (0..map.len())
        .map(|y| {
            (0..map[y].len())
                .filter(|&x| check_if_visible_naive(map, x, y))
                .count()
        })
        .sum()
}

fn max_scenic_score(map: &[Vec<u8>]) -> u32 {
    (0..map.len())
        .map(|y| {
            (0..map[y].len())
                .map(|x| compute_score_naive(map, x, y))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!(
                "{} trees are visible from the outside",
                count_visible(&self.map)
            ),
            Part::Two => format!(
                "max scenic score is: {}",
                max_scenic_score(&self.map)
            ),
        }
    }

    fn day_number() -> u32 {
        8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let solution: Solution = indoc::indoc! {
            "30373
            25512
            65332
            33549
            35390"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 21);

        assert_eq!(max_scenic_score(&solution.map), 8);
    }

    #[test]
    fn test_my_examples() {
        let solution: Solution = indoc::indoc! {
            "30373
            25512
            65032
            33549
            35390"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 21);

        let solution: Solution = indoc::indoc! {
            "00000
            00000
            00000
            00000
            00000"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 16);

        let solution: Solution = indoc::indoc! {
            "00000
            01110
            02220
            03330
            00000"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 25);

        let solution: Solution = indoc::indoc! {
           "00000
            01110
            02120
            03330
            00000"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 24);

        let solution: Solution = indoc::indoc! {
           "9999
            9999
            9999
            9999"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 12);

        let solution: Solution = indoc::indoc! {
           "0000
            9999
            9999
            9999"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 14);

        let solution: Solution = indoc::indoc! {
           "999999
            999999
            999999
            999999
            999999
            999999"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 20);

        let solution: Solution = indoc::indoc! {
           "12345
            23456
            34567
            45678
            56789"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 25);

        let solution: Solution = indoc::indoc! {
           "56789
            45678
            34567
            23456
            12345"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 25);

        let solution: Solution = indoc::indoc! {
           "54321
            65432
            76543
            87654
            98765"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 25);

        let solution: Solution = indoc::indoc! {
           "54999
            65492
            76593
            87694
            98795"
        }
        .parse()
        .unwrap();

        assert_eq!(count_visible(&solution.map), 22);
    }
}
