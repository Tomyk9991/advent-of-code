use std::collections::HashSet;
use std::str::FromStr;
use crate::Error;

#[derive(Default, Clone)]
pub struct Day {
    input: [u32; 2]
}

fn to_digits(n: u32) -> [u8; 6] {
    let mut n = n;
    let mut digits = [0u8; 6];

    for i in (0..6).rev() {
        digits[i] = (n % 10) as u8;
        n /= 10;
    }

    digits
}

impl crate::Day for Day {
    type Output = u32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut hash_set = HashSet::new();

        // its a six-digit number
        // the value is within the range given in your puzzle input.
        for value in self.input[0]..=self.input[1] {
            let digits = to_digits(value);

            // adjacent digits are the same
            let mut same_adjacent_found = false;
            for window in digits.windows(2) {
                if window[0] == window[1] {
                    same_adjacent_found = true;
                    break;
                }
            }

            if !same_adjacent_found {
                continue;
            }

            //going from left to right, the digits never decrease -> monoton steigend
            // check if digits are sorted
            let mut sorted = true;

            for window in digits.windows(2) {
                if window[0] > window[1] {
                    sorted = false;
                    break;
                }
            }

            if !sorted {
                continue;
            }

            hash_set.insert(value);
        }


        Ok(hash_set.len() as u32)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut hash_set = HashSet::new();

        // its a six-digit number
        // the value is within the range given in your puzzle input.
        for value in self.input[0]..=self.input[1] {
            let s: [u8; 6] = to_digits(value);
            let mut digits:[u8; 8] = [0u8; 8];

            digits[1..7].copy_from_slice(&s);

            // adjacent digits are the same
            let mut same_adjacent_found = false;
            for window in digits.windows(4) {
                if (window[0] != window[1]) && (window[1] == window[2]) && (window[2] != window[3]) {
                    same_adjacent_found = true;
                    break;
                }
            }

            if !same_adjacent_found {
                continue;
            }

            //going from left to right, the digits never decrease -> monoton steigend
            // check if digits are sorted
            let digits = to_digits(value);
            let mut sorted = true;

            for window in digits.windows(2) {
                if window[0] > window[1] {
                    sorted = false;
                    break;
                }
            }

            if !sorted {
                continue;
            }

            hash_set.insert(value);
        }


        Ok(hash_set.len() as u32)
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.split('-').collect::<Vec<_>>();
        Ok(Self {
            input: [p[0].parse::<u32>()?, p[1].parse::<u32>()?],
        })
    }
}