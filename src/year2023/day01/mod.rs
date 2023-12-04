use std::str::FromStr;

use crate::Error;

#[derive(Default, Clone)]
pub struct Day {
    lines: Vec<Vec<char>>,
}

impl crate::Day for Day {
    type Output = u32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet", 142)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen", 281)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        Ok(combine_searched_digits(&self.lines))
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let lines = self.lines.iter()
            .map(|char_array| char_array.iter().collect::<String>())
            .map(|line| {
                line.replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e")
            }).map(|a| a.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        Ok(combine_searched_digits(&lines))
    }
}

fn combine_searched_digits(lines: &[Vec<char>]) -> u32 {
    lines.iter().fold(0, |acc, x| {
        let first_digit = x.iter().find(|char| char.is_ascii_digit());
        let last_digit = x.iter().rfind(|char| char.is_ascii_digit());

        if let (Some(first_digit), Some(last_digit)) = (first_digit, last_digit) {
            let first_digit = first_digit.to_digit(10);
            let last_digit = last_digit.to_digit(10);

            if let (Some(first), Some(last)) = (first_digit, last_digit) {
                let combined = first * 10 + last;
                return acc + combined;
            }
        }

        0
    })
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            lines: s.lines()
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<Vec<char>>>(),
        })
    }
}