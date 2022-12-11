use std::{cmp::Reverse, error::Error, str::FromStr};

use advent_utils::{Part, Solver};
use color_eyre::{eyre::eyre, Report};

#[derive(Debug)]
pub struct Solution {
    monkeys: Vec<Monkey>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s.split("\n\n").map(str::parse).collect::<Result<_, _>>()?;

        Ok(Self { monkeys })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        let mut monkeys = self.monkeys.clone();
        let n_rounds = match part {
            Part::One => 20,
            Part::Two => 10000,
        };
        let divide_by = match part {
            Part::One => 3,
            Part::Two => 1,
        };
        let modulo =
            lcm(monkeys.iter().map(|m| m.divisible_by)).expect("no monkeys in the game :(");

        for _ in 0..n_rounds {
            for i in 0..monkeys.len() {
                for (new_monkey, item) in monkeys[i].make_move(divide_by, modulo) {
                    monkeys[new_monkey].items.push(item);
                }
            }
        }

        monkeys.sort_unstable_by_key(|m| Reverse(m.business));

        format!(
            "monkey business is {}",
            monkeys[0].business * monkeys[1].business
        )
    }

    fn day_number() -> u32 {
        11
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
    business: usize,
}

impl Monkey {
    fn make_move(&mut self, divide_by: u64, modulo: u64) -> Vec<(usize, u64)> {
        self.items
            .drain(..)
            .map(|item| {
                self.business += 1;

                let new_item = ((self.operation.apply(item) / divide_by) % modulo) as u64;

                if new_item % self.divisible_by == 0 {
                    (self.if_true, new_item)
                } else {
                    (self.if_false, new_item)
                }
            })
            .collect()
    }
}

impl FromStr for Monkey {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || eyre!("unknown monkey format: {s}");

        let (_, rest) = s.split_once('\n').ok_or_else(err)?;

        // Starting items: 1, 5
        let (items_str, rest) = rest
            .strip_prefix("  Starting items: ")
            .and_then(|s| s.split_once('\n'))
            .ok_or_else(err)?;
        let items = items_str
            .split(", ")
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        // Operation: old * 5
        let (operation_str, rest) = rest
            .strip_prefix("  Operation: new = ")
            .and_then(|s| s.split_once('\n'))
            .ok_or_else(err)?;
        let operation = operation_str.parse()?;

        // Operation: old * 5
        let (test_str, rest) = rest
            .strip_prefix("  Test: divisible by ")
            .and_then(|s| s.split_once('\n'))
            .ok_or_else(err)?;
        let divisible_by = test_str.parse()?;

        // If true: throw to monkey 3
        let (if_true, rest) = rest
            .strip_prefix("    If true: throw to monkey ")
            .and_then(|s| s.split_once('\n'))
            .ok_or_else(err)?;
        let if_true = if_true.parse()?;

        // If false: throw to monkey 6
        let if_false = rest
            .strip_prefix("    If false: throw to monkey ")
            .ok_or_else(err)?
            .parse()?;

        Ok(Self {
            operation,
            items,
            divisible_by,
            if_true,
            if_false,
            business: 0,
        })
    }
}

#[derive(Debug, Clone)]
struct Operation {
    left: Operand,
    right: Operand,
    operator: Operator,
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        let left = match self.left {
            Operand::Item => old,
            Operand::Const(n) => n,
        } as u64;

        let right = match self.right {
            Operand::Item => old,
            Operand::Const(n) => n,
        } as u64;

        match self.operator {
            Operator::Add => left + right,
            Operator::Mul => left * right,
        }
    }
}

impl FromStr for Operation {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || eyre!("unknown operation format: {s}");

        let (left, rest) = s.split_once(' ').ok_or_else(err)?;
        let (operator, right) = rest.split_once(' ').ok_or_else(err)?;

        Ok(Self {
            left: left.parse()?,
            right: right.parse()?,
            operator: operator.parse()?,
        })
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Item,
    Const(u64),
}

impl FromStr for Operand {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Item),
            n => Ok(Self::Const(n.parse()?)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Mul,
    Add,
}

impl FromStr for Operator {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(eyre!("unknown operator: {s}")),
        }
    }
}

fn lcm(nums: impl Iterator<Item = u64>) -> Option<u64> {
    use gcd::binary_u64;

    nums.reduce(|a, b| a * b / binary_u64(a, b))
}
