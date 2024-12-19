use std::collections::{HashMap};
use std::str::FromStr;

#[derive(Default, Clone, Debug)]
pub struct Day {
    rules: Vec<String>,
    targets: Vec<String>,
}


impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = Vec::new();

        let mut lines = s.lines();
        if let Some(line) = lines.next() {
            rules = line.split(",").map(|a| a.trim().to_string()).collect::<Vec<_>>()
        }

        let messages = lines.collect::<Vec<_>>();

        Ok(Self {
            rules,
            targets: messages.iter().filter(|t| !t.trim().is_empty()).map(|s| s.to_string()).collect::<Vec<_>>(),
        })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb", 6)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb", 16)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut cache = HashMap::new();
        Ok(self.targets.iter().map(|target| {
            let patterns = self.rules.clone();
            if contains(target, &patterns, &mut cache) {
                1
            } else {
                0
            }
        }).sum())
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut cache = HashMap::new();
        Ok(self.targets.iter().map(|target| {
            contains_count(target, &self.rules, &mut cache)
        }).sum())
    }
}


fn contains_count(target: &str, patterns: &Vec<String>, cache: &mut HashMap<String, u64>) -> u64 {
    if target.is_empty() {
        return 1;
    }

    if let Some(&result) = cache.get(target) {
        return result;
    }

    let mut count = 0;
    for i in 0..(target.len().min(patterns.len()) + 1) {
        if patterns.contains(&target[..i].to_string()) {
            let inner_count = contains_count(&target[i..], patterns, cache);;
            cache.insert(target[i..].to_string(), inner_count);
            count += contains_count(&target[i..], patterns, cache);
        }
    }

    cache.insert(target.to_string(), count);
    count
}

fn contains(target: &str, patterns: &Vec<String>, cache: &mut HashMap<String, bool>) -> bool {
    if target.is_empty() {
        return true;
    }
    if let Some(&result) = cache.get(target) {
        return result;
    }

    for i in 0..(target.len().min(patterns.len()) + 1) {
        if patterns.contains(&target[..i].to_string()) && contains(&target[i..], patterns, cache) {
            cache.insert(target.to_string(), true);
            return true;
        }
    }

    cache.insert(target.to_string(), false);
    false
}