use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};
use color_eyre::Result;
use fnv::FnvHashSet;

#[derive(Debug)]
pub struct Solution {
    stream: String,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            stream: s.to_owned(),
        })
    }
}

fn find_packet_start(stream: &str, packet_size: usize) -> Option<usize> {
    let bytes: Vec<_> = stream.bytes().collect();

    bytes
        .windows(packet_size)
        .enumerate()
        .find_map(|(i, window)| {
            let set = FnvHashSet::from_iter(window);

            (set.len() == packet_size).then_some(i + packet_size)
        })
}

impl Solver for Solution {
    fn day_number() -> u32 {
        6
    }

    fn solve(&self, part: Part) -> String {
        let packet_size = match part {
            Part::One => 4,
            Part::Two => 14,
        };

        let start =
            find_packet_start(&self.stream, packet_size).expect("couldn't find packet start");

        format!("packet starts at {start}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            find_packet_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
            Some(7)
        );
    }
}
