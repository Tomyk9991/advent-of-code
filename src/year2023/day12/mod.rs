use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use itertools::Itertools;

use crate::aoc::Error;

#[derive(Debug, Clone, Default)]
struct Line {
    raw: String,
    target: Vec<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct Day {
    lines: Vec<Line>,
}

fn count_combinations<'a>(config: &'a str, numbers: &'a [usize], cache: &mut HashMap<(&'a str, &'a [usize]), usize>) -> usize {
    if config.is_empty() {
        return if numbers.len() == 0 { 1 } else { 0 }
    }

    if numbers.is_empty() {
        return if config.contains('#') { 0 } else { 1 }
    }

    let key = (config, numbers);

    if let Some(value) = cache.get(&key) {
        return *value;
    }

    let mut result = 0;

    if let Some(first) = config.chars().nth(0) {
        if ".?".contains(first) {
            result += count_combinations(&config[1..], numbers, cache);
        }

        if "#?".contains(first) {
            if numbers[0] <= config.chars().count() && !config[..numbers[0]].contains('.') && (numbers[0] == config.len() || config.chars().nth(numbers[0]).unwrap() != '#') {
                let s = if numbers[0] + 1 >= config.len() {
                    ""
                } else {
                    &config[numbers[0] + 1..]
                };
                result += count_combinations(s, &numbers[1..], cache);
            }
        }
    }


    cache.insert(key, result);
    return result;
}

fn repeat_and_separate(s: &str, count: usize, separator: &str) -> String {
    std::iter::repeat(s)
        .take(count)
        .collect::<Vec<&str>>()
        .join(separator)
}

fn repeat_vec(v: &[usize], count: usize) -> Vec<usize> {
    std::iter::repeat(v)
        .take(count)
        .flatten()
        .cloned()
        .collect()
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1", 21)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1", 525152)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut sum = 0;

        for line in &self.lines {
            let combinations = count_combinations(&line.raw, &line.target, &mut HashMap::new());
            sum += combinations;
        }

        Ok(sum)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut sum = 0;

        for line in &self.lines {
            let l = repeat_and_separate(&line.raw, 5, "?");
            let target = repeat_vec(&line.target, 5);
            let combinations = count_combinations(&l, &target, &mut HashMap::new());
            sum += combinations;
        }

        Ok(sum)
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(|line| {
            let s = line.split(' ').collect::<Vec<_>>();
            Line {
                raw: s[0].to_string(),
                target: s[1].split(',').map(|number| number.parse::<usize>().unwrap_or(0)).collect(),
            }
        }).collect::<Vec<_>>();
        Ok(Self {
            lines,
        })
    }
}