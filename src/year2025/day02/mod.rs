use std::str::FromStr;

#[derive(Default, Clone, Debug)]
pub struct Day {
    product_ranges: Vec<(String, String)>,
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            product_ranges: {
                let parts = s.split(',').collect::<Vec<&str>>();

                parts.iter().filter(|p| !p.trim().is_empty()).map(|p| {
                    let id = p.split("-").collect::<Vec<&str>>();
                    (id[0].to_string(), id[1].to_string())
                }).collect::<Vec<(String, String)>>()
            },
        })
    }
}


impl crate::aoc::Day for Day {
    type Output = i64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124", 1227775554)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124", 4174379265)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut invalid_numbers = vec![];
        for (lower, upper) in &self.product_ranges {
            let lower = lower.parse::<i64>()?;
            let upper = upper.parse::<i64>()?;
            for num in lower..=upper {
                if has_repeating_sequence_twice(&num.to_string()) {
                    invalid_numbers.push(num);
                }
            }
        }

        Ok(invalid_numbers.iter().sum())
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut invalid_numbers = vec![];
        for (lower, upper) in &self.product_ranges {
            let lower = lower.parse::<i64>()?;
            let upper = upper.parse::<i64>()?;
            for num in lower..=upper {
                if has_repeating_sequence(&num.to_string()) {
                    invalid_numbers.push(num);
                }
            }
        }

        Ok(invalid_numbers.iter().sum())
    }
}

pub fn has_repeating_sequence(num: &str) -> bool {
    let bytes = num.as_bytes();
    let n = bytes.len();

    for block_len in 1..=n / 2 {
        if n % block_len != 0 {
            continue;
        }

        let block = &bytes[..block_len];
        let repeats = n / block_len;

        if repeats >= 2 {
            let mut ok = true;

            for i in 1..repeats {
                let start = i * block_len;
                if &bytes[start..start + block_len] != block {
                    ok = false;
                    break;
                }
            }

            if ok {
                return true;
            }
        }
    }

    false
}


pub fn has_repeating_sequence_twice(num: &str) -> bool {
    let n = num.len();
    if n % 2 != 0 {
        return false;
    }

    let half = n / 2;
    num[..half] == num[half..]
}