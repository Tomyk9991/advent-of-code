use std::collections::HashMap;
use std::str::FromStr;
use crate::aoc::Error;

#[derive(Default, Clone)]
pub struct Day {
    left_list: Vec<i32>,
    right_list: Vec<i32>
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left_list = Vec::new();
        let mut right_list = Vec::new();

        for line in s.lines() {
            let mut parts = line.split_whitespace();
            left_list.push(parts.next().ok_or(Error::StringParse("Missing left value".to_string()))?.parse()?);
            right_list.push(parts.next().ok_or(Error::StringParse("Missing right value".to_string()))?.parse()?);
        }

        Ok(Self { left_list, right_list })
    }
}

impl crate::aoc::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n", 11)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n", 31)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut left_sorted = self.left_list.clone();
        left_sorted.sort();

        let mut right_sorted = self.right_list.clone();
        right_sorted.sort();

        Ok(left_sorted.iter().zip(right_sorted.iter()).fold(0, |acc, (left_element, right_element)| {
            acc + (left_element - right_element).abs()
        }))
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        // build a hashset, where you count the number of times a value from the left appears in the right list
        let mut right_set = self.right_list.iter().fold(HashMap::new(), |mut acc, right| {
            let count = acc.entry(*right).or_insert(0);
            *count += 1;
            acc
        });

        Ok(self.left_list.iter().fold(0, |acc, left| {
            acc + (left * right_set.get(left).unwrap_or(&0))
        }))
    }
}