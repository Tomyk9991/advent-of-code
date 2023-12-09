use std::str::FromStr;

use crate::aoc::Error;

#[derive(Debug, Clone, Default)]
struct Sequence {
    values: Vec<i64>,
}

impl Sequence {
    fn generate_sequence(&self) -> (Vec<Vec<i64>>, Vec<i64>) {
        let mut sequences = vec![self.values.clone()];
        let mut last_sequence = self.values.clone();

        while last_sequence.iter().any(|x| *x != 0) {
            let mut new_sequence = Vec::new();

            for i in 1..last_sequence.len() {
                new_sequence.push(last_sequence[i] - last_sequence[i - 1]);
            }

            sequences.push(new_sequence.clone());
            last_sequence = new_sequence;
        }

        (sequences, last_sequence)
    }
    fn extrapolate(&self) -> i64 {
        let (sequences, mut last_sequence) = self.generate_sequence();

        for sequence in sequences.iter().rev() {
            let last_value = *sequence.last().unwrap_or(&0);
            let extrapolated_value = *last_sequence.last().unwrap_or(&0);
            last_sequence.push(last_value + extrapolated_value);
        }

        *last_sequence.last().unwrap_or(&0)
    }

    fn extrapolate_front(&self) -> i64 {
        let (sequences, mut last_sequence) = self.generate_sequence();

        for sequence in sequences.iter().rev() {
            let first_value = *sequence.first().unwrap_or(&0);
            last_sequence.insert(0, first_value - last_sequence[0]);
        }

        *last_sequence.first().unwrap_or(&0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Day {
    sequences: Vec<Sequence>,
}


impl crate::aoc::Day for Day {
    type Output = i64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45", 114)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45", 2)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let sum = self.sequences.iter().fold(0, |a, b| a + b.extrapolate());
        Ok(sum)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let sum = self.sequences.iter().fold(0, |a, b| a + b.extrapolate_front());
        Ok(sum)
    }
}


impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sequences: s.lines().map(|l| Sequence { values: l.split(' ').map(|a| a.parse::<i64>().unwrap_or(0)).collect::<Vec<_>>() }).collect::<Vec<_>>()
        })
    }
}