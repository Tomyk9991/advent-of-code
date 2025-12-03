use std::fmt::Debug;
use std::str::FromStr;

#[derive(Default, Clone, Debug)]
pub struct Day {
    banks: String,
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            banks: s.to_string()
        })
    }
}


impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("987654321111111
811111111111119
234234234234278
818181911112111", 357)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("987654321111111
811111111111119
234234234234278
818181911112111", 3121910778619)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let s: Vec<&[u8]> = self.banks.lines().map(str::as_bytes).collect();
        Ok(solve(&s, 2))
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let s: Vec<&[u8]> = self.banks.lines().map(str::as_bytes).collect();
        Ok(solve(&s, 12))
    }
}

fn solve(banks: &[&[u8]], limit: usize) -> u64 {
    banks
        .iter()
        .map(|bank| {
            let mut max = 0;
            let mut start = 0;

            (0..limit).fold(0, |joltage, digit| {
                (max, start) = (start..bank.len() - limit + digit + 1).fold((0, 0), |(max, start), i| {
                    if bank[i] > max { (bank[i], i + 1) } else { (max, start) }
                });
                10 * joltage + (max - b'0') as u64
            })
        })
        .sum()
}