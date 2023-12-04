use std::str::FromStr;
use crate::Error;
use crate::year2019::day05::int_code_computer::{IntCodeComputer, IO};

mod int_code_computer;

#[derive(Default, Clone)]
pub struct Day {
    pub values: Vec<i32>,
    pub parameter: Option<(i32, i32)>
}

impl crate::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec!(("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 999))
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let program = &mut self.values;
        let mut computer = IntCodeComputer::new(program.clone(), IO { value: 1 });
        computer.run();

        Ok(computer.io.value)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let program = &mut self.values;
        let mut computer = IntCodeComputer::new(program.clone(), IO { value: 5 });
        computer.run();

        Ok(computer.io.value)
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s.split(',')
                .map(|s| s.parse::<i32>())
                .filter_map(|s| s.ok())
                .collect(),
            parameter: None
        })
    }
}