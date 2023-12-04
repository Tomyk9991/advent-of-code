use std::str::FromStr;

use crate::Error;

#[derive(Default, Clone)]
pub struct Day {
    values: Vec<u32>,
}

impl crate::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("12", 2),
            ("14", 2),
            ("1969", 654),
            ("100756", 33583),
        ]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("14", 2),
            ("1969", 966),
            ("100756", 50346),
        ]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        return Ok(self.values.iter().map(|s| s / 3 - 2).sum::<u32>() as i32);
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        return Ok(self.values.iter().map(|s| {
            let value = (*s as i32) / 3 - 2;
            let mut negative_fuel = value / 3 - 2;
            let mut sum = value;

            while negative_fuel > 0 {
                sum += negative_fuel;
                negative_fuel = negative_fuel / 3 - 2;
            }

            if negative_fuel > 0 {
                sum += negative_fuel;
            }

            sum
        }).sum::<i32>());
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s.lines()
                .map(|s| s.parse::<u32>())
                .filter_map(|k| k.ok())
                .collect::<Vec<_>>(),
        })
    }
}