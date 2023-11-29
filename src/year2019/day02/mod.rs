use std::str::FromStr;

use crate::Error;

#[derive(Default, Clone)]
pub struct Day {
    pub values: Vec<usize>,
    pub parameter: Option<(usize, usize)>
}

impl crate::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("1,9,10,3,2,3,11,0,99,30,40,50", 3500),
            ("1,0,0,0,99", 2),
            ("2,3,0,3,99", 2),
            ("2,4,4,5,99,0", 2),
            ("1,1,1,4,99,5,6,0,99", 30)
        ]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![]
    }

    fn after_test_2(&mut self) {
        self.parameter = None;
    }

    fn after_test_1(&mut self) {
        self.parameter = Some((12, 2));
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let program = &mut self.values;
        let mut index = 0;

        if let Some(parameter) = self.parameter {
            program[1] = parameter.0;
            program[2] = parameter.1;
        }

        loop {
            let opt_code = program[index];
            match opt_code {
                1 => {
                    let window = program[index..index + 4].iter().copied().clone().collect::<Vec<usize>>();

                    if let [_, position_one, position_two, target] = window[..] {
                        program[target] = program[position_one] + program[position_two];
                    }
                }
                2 => {
                    let window = program[index..index + 4].iter().copied().clone().collect::<Vec<usize>>();
                    if let [_, position_one, position_two, target] = window[..] {
                        program[target] = program[position_one] * program[position_two];
                    }
                }
                99 => break,
                _ => {},
            }

            index += 4;
        }

        Ok(program[0] as i32)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let target_string = 19690720;

        for i in 0..=99 {
            for j in 0..=99 {
                let mut c = Day::from_str(include_str!("./input.txt"))?;
                c.parameter = Some((i, j));

                if target_string == c.solution1()? {
                    return Ok((100 * i + j) as i32);
                }
            }
        }

        Err(Error::NoSolutionFound.into())
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s.split(',')
                .map(|s| s.parse::<usize>())
                .filter_map(|s| s.ok())
                .collect(),
            parameter: None
        })
    }
}