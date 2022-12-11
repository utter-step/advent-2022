use std::{
    collections::HashSet,
    error::Error,
    fmt,
    iter::{empty, repeat},
    str::{Bytes, FromStr},
};

use advent_utils::{Part, Solver};
use color_eyre::{eyre::eyre, Report};
use lending_iterator::{windows_mut, LendingIterator};

type Set<T> = HashSet<T>;

#[derive(Debug)]
pub struct Solution {
    moves: Vec<Move>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s.lines().map(str::parse).collect::<Result<_, _>>()?;

        Ok(Self { moves })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let mut rope = Rope::new(2);
                let mut visited = Set::new();
                visited.insert((0, 0));

                self.moves
                    .iter()
                    .for_each(|&move_| visited.extend(rope.make_move(move_)));

                format!("tail visited {} unique positions", visited.len())
            }
            Part::Two => {
                let mut rope = Rope::new(10);
                let mut visited = Set::new();
                visited.insert((0, 0));

                self.moves
                    .iter()
                    .for_each(|&move_| visited.extend(rope.make_move(move_)));

                format!(
                    "tail of 10-segmented rope visited {} unique positions",
                    visited.len()
                )
            }
        }
    }

    fn day_number() -> u32 {
        9
    }

    fn implemented_parts() -> Vec<Part> {
        vec![Part::One]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Left(i64),
    Right(i64),
    Up(i64),
    Down(i64),
}

impl FromStr for Move {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((dir, n)) = s.split_once(' ') {
            let nsteps = n.parse()?;

            match dir {
                "L" => Ok(Self::Left(nsteps)),
                "R" => Ok(Self::Right(nsteps)),
                "D" => Ok(Self::Down(nsteps)),
                "U" => Ok(Self::Up(nsteps)),
                other => Err(eyre!("unknown move: {other}")),
            }
        } else {
            Err(eyre!("unknown move format: {s}"))
        }
    }
}

#[derive(Debug)]
struct Rope {
    segments: Vec<Point>,
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = self.segments.iter().map(|p| p.x).min().unwrap_or_default();
        let max_x = self.segments.iter().map(|p| p.x).max().unwrap_or_default();
        let min_y = self.segments.iter().map(|p| p.y).min().unwrap_or_default();
        let max_y = self.segments.iter().map(|p| p.y).max().unwrap_or_default();

        let mut map: Vec<_> = (min_y..=max_y)
            .map(|_| vec![b'.'; (max_x - min_x + 1) as usize])
            .collect();

        for (i, segment) in self.segments.iter().enumerate().rev() {
            map[(segment.y - min_y) as usize][(segment.x - min_x) as usize] =
                if i == 0 { b'H' } else { b'0' + i as u8 }
        }

        let result = map
            .into_iter()
            .reduce(|line_a, line_b| [line_a, vec![b'\n'], line_b].concat())
            .and_then(|map| String::from_utf8(map).ok())
            .unwrap_or_default();

        write!(f, "{result}")
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Rope {
    pub fn new(n_segments: usize) -> Self {
        Self {
            segments: vec![Default::default(); n_segments],
        }
    }

    pub fn make_move(&mut self, move_: Move) -> impl Iterator<Item = (i64, i64)> {
        println!("Making move {move_:?}, self state: \n{self}");

        match move_ {
            Move::Left(nsteps) => self.segments[0].x -= nsteps,
            Move::Right(nsteps) => self.segments[0].x += nsteps,
            Move::Up(nsteps) => self.segments[0].y -= nsteps,
            Move::Down(nsteps) => self.segments[0].y += nsteps,
        }

        self.pull_tail()
    }

    fn pull_tail(&mut self) -> Box<dyn Iterator<Item = (i64, i64)>> {
        let mut iter: Box<dyn Iterator<Item = (i64, i64)>> = Box::new(empty());
        let mut windows = windows_mut::<_, 2>(&mut self.segments);

        while let Some([head, tail]) = windows.next() {
            iter = Box::new(Rope::pull_segment(head, tail));
        }

        iter
    }

    fn pull_segment(head: &Point, tail: &mut Point) -> Box<dyn Iterator<Item = (i64, i64)>> {
        let head_x = head.x;
        let head_y = head.y;
        let tail_x = tail.x;
        let tail_y = tail.y;

        match (head_x.abs_diff(tail_x), head_y.abs_diff(tail_y)) {
            (0, 0) | (1, 0) | (0, 1) | (1, 1) => Box::new(empty()),
            (dx, dy) => {
                let x_iter: Box<dyn Iterator<Item = _>> = if head_x > tail_x {
                    if dx > dy {
                        tail.x = head_x - 1;

                        Box::new(tail_x + 1..=tail.x)
                    } else {
                        tail.x = head_x;

                        Box::new((tail_x + 1..=tail.x).chain(repeat(tail.x)))
                    }
                } else if dx > dy {
                    tail.x = head_x + 1;

                    Box::new((tail.x..=tail_x - 1).rev())
                } else {
                    tail.x = head_x;

                    Box::new((tail.x..=tail_x - 1).rev().chain(repeat(tail.x)))
                };

                let y_iter: Box<dyn Iterator<Item = _>> = if head_y > tail_y {
                    if dx <= dy {
                        tail.y = head_y - 1;

                        Box::new(tail_y + 1..=tail.y)
                    } else {
                        tail.y = head_y;

                        Box::new((tail_y + 1..=tail.y).chain(repeat(tail.y)))
                    }
                } else if dx <= dy {
                    tail.y = head_y + 1;

                    Box::new((tail.y..=tail_y - 1).rev())
                } else {
                    tail.y = head_y;

                    Box::new((tail.y..=tail_y - 1).rev().chain(repeat(tail.y)))
                };

                Box::new(x_iter.zip(y_iter))
            }
        }
    }
}

// impl fmt::Display for Rope {

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let solution: Solution = indoc::indoc! {
            "R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2"
        }
        .parse()
        .unwrap();

        assert_eq!(
            solution.solve(Part::One),
            "tail visited 13 unique positions"
        );
        assert_eq!(
            solution.solve(Part::Two),
            "tail of 10-segmented rope visited 1 unique positions"
        );
    }

    #[test]
    #[ignore = "not yet debugged"]
    fn test_example_two() {
        let solution: Solution = indoc::indoc! {
            "R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20"
        }
        .parse()
        .unwrap();

        assert_eq!(
            solution.solve(Part::Two),
            "tail of 10-segmented rope visited 36 unique positions"
        );
    }

    #[test]
    #[ignore = "not yet debugged"]
    fn test_my_examples() {
        let solution: Solution = indoc::indoc! {
            "R 5"
        }
        .parse()
        .unwrap();

        let mut rope = Rope::new(10);
        let mut visited = vec![(0, 0)];
        solution
            .moves
            .iter()
            .for_each(|&move_| visited.extend(rope.make_move(move_)));

        assert_eq!(visited.len(), 1, "{visited:?}");

        let solution: Solution = indoc::indoc! {
            "R 1
            U 1
            D 2
            L 2
            D 1
            U 1
            R 2
            D 2
            R 2
            U 1
            L 1
            U 2
            R 1
            L 1
            R 1
            L 1
            R 1
            U 1
            L 1
            U 1
            R 2
            U 2
            L 1
            R 1
            L 1
            U 2
            R 2
            D 1
            R 2
            U 2
            L 1
            U 2
            D 2
            L 1
            U 2
            R 2
            L 2
            U 1
            D 1
            L 1
            R 1
            L 1
            U 1
            L 1
            U 1
            R 2
            L 1
            R 1
            D 2
            L 2
            R 2
            U 2
            D 1
            L 1
            U 2
            R 2
            D 1
            U 1
            D 2
            U 1
            L 1
            D 2
            U 1
            R 1
            L 1
            U 2
            R 2
            U 1
            D 2
            R 1
            L 2
            D 2
            L 1
            D 2
            L 1
            R 1
            U 1
            R 2
            L 2
            U 2
            L 2
            R 1
            L 1
            D 1
            L 1
            R 1
            L 1
            R 1
            U 2
            D 1
            L 1
            R 1
            L 2
            U 2
            D 1
            U 1
            R 1
            U 1
            L 2
            U 2
            R 2
            U 2
            L 1
            U 1
            R 1
            U 2
            R 2
            U 1
            L 2
            U 2
            L 1
            D 1"
        }
        .parse()
        .unwrap();

        let mut rope = Rope::new(10);
        let mut visited = vec![(0, 0)];
        solution
            .moves
            .iter()
            .for_each(|&move_| visited.extend(rope.make_move(move_)));

        assert_eq!(15, visited.len(), "{visited:?}");
    }
}
