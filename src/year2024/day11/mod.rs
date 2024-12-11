use std::collections::{HashMap};

use std::str::FromStr;

#[derive(Default, Clone)]
pub struct Day {
    list: Vec<u64>
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { list: s.split_whitespace().filter_map(|s| s.parse().ok()).collect() })
    }
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("125 17", 55312)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("125 17", 65601038650482)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut hash_map: HashMap<(u64, u32), u64> = HashMap::new();
        Ok(self.list.iter().fold(0, |acc, x| acc + solve(*x, 25, &mut hash_map).unwrap()) as usize)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut hash_map: HashMap<(u64, u32), u64> = HashMap::new();
        Ok(self.list.iter().fold(0, |acc, x| acc + solve(*x, 75, &mut hash_map).unwrap()) as usize)
    }
}
fn solve(x: u64, t: u32, hash_map: &mut HashMap<(u64, u32), u64>) -> anyhow::Result<u64> {
    if let Some(result) = hash_map.get(&(x, t)) {
        return Ok(*result);
    }

    let ret = match (t, x) {
        (0, _) => 1,
        (_, 0) => solve(1, t - 1, hash_map)?,
        (_, x) if x.to_string().len() % 2 == 0 => {
            let d_str = x.to_string();
            let left = d_str[..d_str.len() / 2].parse::<u64>()?;
            let right = d_str[d_str.len() / 2..].parse::<u64>()?;
            solve(left, t - 1, hash_map)? + solve(right, t - 1, hash_map)?
        },
        (_, x) => solve(x * 2024, t - 1, hash_map)?
    };

    hash_map.insert((x, t), ret);
    Ok(ret)
}