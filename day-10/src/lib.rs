use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};
use color_eyre::{eyre::eyre, Report};

#[derive(Debug)]
pub struct Solution {
    cpu: CPU,
    instructions: Vec<Instruction>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s.lines().map(str::parse).collect::<Result<_, _>>()?;

        Ok(Self {
            instructions,
            cpu: CPU::new(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CPU {
    x: i64,
    num_cycle: usize,
    current_instruction: Option<Instruction>,
    instruction_running_for: usize,
}

impl CPU {
    fn new() -> Self {
        Self {
            x: 1,
            num_cycle: 1,
            current_instruction: None,
            instruction_running_for: 0,
        }
    }

    fn retire(&mut self, instruction: Instruction) {
        if self.current_instruction.is_none() {
            self.current_instruction = Some(instruction);
            self.instruction_running_for = 0;
        } else {
            panic!("invalid operation")
        }
    }

    fn try_cycle(&mut self) -> Option<()> {
        match self.current_instruction.take()? {
            Instruction::Noop => {}
            instr @ Instruction::Addx(x) => {
                self.instruction_running_for += 1;
                if self.instruction_running_for < 2 {
                    self.current_instruction = Some(instr);
                } else {
                    self.x += x;
                }
            }
        }

        self.num_cycle += 1;

        Some(())
    }

    fn current_cycle(&self) -> usize {
        self.num_cycle
    }

    fn current_power(&self) -> i64 {
        self.x * self.num_cycle as i64
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Addx(i64),
    Noop,
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4] {
            "addx" => Ok(Self::Addx(s[5..].parse()?)),
            "noop" => Ok(Self::Noop),
            _ => Err(eyre!("unknown instruction: {s}")),
        }
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        let mut cpu = self.cpu.clone();
        let mut instructions = self.instructions.iter();
        let mut total_power = 0;

        match part {
            Part::One => {
                while cpu.num_cycle <= 220 {
                    if cpu.current_cycle() % 40 == 20 {
                        total_power += cpu.current_power() as i64;
                    }

                    if cpu.try_cycle().is_none() {
                        cpu.retire(*instructions.next().expect("not enough instructions"));
                        cpu.try_cycle().expect("failed to run retired instruction");
                    }
                }

                format!("total power is {total_power}")
            }
            Part::Two => {
                let mut result = String::with_capacity(250);

                while cpu.num_cycle <= 240 {
                    if cpu.x.abs_diff((cpu.current_cycle() - 1) as i64 % 40) <= 1 {
                        result.push('#');
                    } else {
                        result.push('.');
                    }

                    if cpu.try_cycle().is_none() {
                        cpu.retire(*instructions.next().expect("not enough instructions"));
                        cpu.try_cycle().expect("failed to run retired instruction");
                    }

                    if cpu.current_cycle() % 40 == 1 {
                        result.push('\n');
                    }
                }

                format!("CRT reads:\n{result}")
            }
        }
    }

    fn day_number() -> u32 {
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let solution: Solution = include_str!("../example.txt").parse().unwrap();

        assert_eq!(solution.solve(Part::One), "total power is 13140");
        assert_eq!(
            solution.solve(Part::Two),
            indoc::indoc! {
                "CRT reads:
                ##..##..##..##..##..##..##..##..##..##..
                ###...###...###...###...###...###...###.
                ####....####....####....####....####....
                #####.....#####.....#####.....#####.....
                ######......######......######......####
                #######.......#######.......#######.....
                "
            }
        );
    }
}
