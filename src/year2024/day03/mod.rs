use std::str::FromStr;
use regex::Regex;
use crate::aoc::Error;

#[derive(Default, Clone)]
enum Instruction {
    Mul(u32, u32),
    #[default]
    Do,
    Dont
}

#[derive(Default, Clone)]
pub struct Day {
    instructions: Vec<Instruction>,
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")?;
        let mut instructions = vec![];

        for cap in re.captures_iter(s) {
            if let Some(capture_match) = cap.get(0) {
                match &capture_match.as_str()[0..3] {
                    "mul" => {
                        let arg1 = cap.get(1).map_or("", |m| m.as_str()).parse()?;
                        let arg2 = cap.get(2).map_or("", |m| m.as_str()).parse()?;
                        instructions.push(Instruction::Mul(arg1, arg2));
                    },
                    "do(" => instructions.push(Instruction::Do),
                    "don" => instructions.push(Instruction::Dont),
                    _ => {}
                }
            }
        }

        Ok(Self {
            instructions
        })
    }
}

impl crate::aoc::Day for Day {
    type Output = u32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", 161)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", 48)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        Ok(self.instructions.iter().fold(0, |acc, instruction| acc + {
            match instruction {
                Instruction::Mul(a, b) => a * b,
                Instruction::Do => 0,
                Instruction::Dont => 0
            }
        }))
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut ignore_next_mul = false;
        Ok(self.instructions.iter().fold(0, |acc, instruction| acc + {
            match instruction {
                Instruction::Mul(a, b) if !ignore_next_mul => a * b,
                Instruction::Do => { ignore_next_mul = false; 0 },
                Instruction::Dont => { ignore_next_mul = true; 0 }
                _ => 0
            }
        }))
    }
}