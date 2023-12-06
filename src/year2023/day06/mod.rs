use std::str::FromStr;

use crate::aoc::Error;

#[derive(Debug, Default, Clone)]
pub struct Day {
    milliseconds: Vec<u32>,
    distances: Vec<u32>,
}

impl crate::aoc::Day for Day {
    type Output = String;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("Time:      7  15   30\nDistance:  9  40  200", "288".to_string())]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("Time:      7  15   30\nDistance:  9  40  200", "71503".to_string())]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let races = self.milliseconds.len();
        let mut product = 1;

        for i in 0..races {
            let time_for_whole_track = self.milliseconds[i];
            let distance_to_beat = self.distances[i];

            let mut amount_won = 0;
            for speed in 1..time_for_whole_track {
                let distance_traveled = speed * (time_for_whole_track - speed);

                if distance_traveled > distance_to_beat {
                    amount_won += 1;
                }
            }

            product *= amount_won;
        }

        Ok(product.to_string())
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let milliseconds = self.milliseconds.iter().map(|s| s.to_string()).collect::<String>().parse::<u128>()?;
        let distance = self.distances.iter().map(|s| s.to_string()).collect::<String>().parse::<u128>()?;

        let time_for_whole_track = milliseconds;
        let distance_to_beat = distance;

        let mut amount_won = 0;

        for speed in 1..time_for_whole_track {
            let distance_traveled = speed * (time_for_whole_track - speed);

            if distance_traveled > distance_to_beat {
                amount_won += 1;
            }
        }

        Ok(amount_won.to_string())
    }
}


impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        if let (
            ["Time:", times @ ..],
            ["Distance:", distances @ ..]
        ) = (
            &lines[0].split_whitespace().collect::<Vec<_>>()[..],
            &lines[1].split_whitespace().collect::<Vec<_>>()[..]
        ) {
            return Ok(Self {
                milliseconds: times.iter().filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<_>>(),
                distances: distances.iter().filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<_>>()
            });
        }

        Err(Error::StringParse(s.to_string()))
    }
}