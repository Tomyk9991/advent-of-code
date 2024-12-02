use std::str::FromStr;
use crate::aoc::Error;

#[derive(Default, Clone)]
pub struct Day {
    reports: Vec<Vec<u32>>,
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reports = s.lines()
            .map(|line| {
                line.split(' ')
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        Ok(Self { reports })
    }
}

impl crate::aoc::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#, 2)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#, 4)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut safe = 0;

        for report in &self.reports {
            if report_save(report) {
                safe += 1;
            }
        }

        Ok(safe)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut safe = 0;
        for report in &self.reports {

            if report_save(report) {
                safe += 1;
                continue;
            }

            'outer: for i in 0..report.len() {
                let mut subset_report = report.clone();
                subset_report.remove(i);

                if report_save(&subset_report) {
                    safe += 1;
                    break 'outer;
                }
            }
        }

        Ok(safe)
    }
}

fn report_save(report: &[u32]) -> bool {
    let mut direction = 0;
    for window in report.windows(2) {
        if let [a, b] = *window {
            match [a, b] {
                [a, b] if ((a as i32) - (b as i32)).abs() > 3 => {
                    return false;
                }
                [a, b] if (a < b && (b - a) > 0) && (direction == 0 || direction == 1) => {
                    direction = 1;
                }
                [a, b] if (a > b && (a - b) > 0) && (direction == 0 || direction == -1) => {
                    direction = -1;
                }
                _ => {
                    return false;
                }
            }
        }
    }

    true
}