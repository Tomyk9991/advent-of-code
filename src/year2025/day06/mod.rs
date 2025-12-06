use std::str::FromStr;
use crate::utils::grid::{Grid};

#[derive(Default, Clone, Debug)]
pub struct Day {
    calculation: Vec<Calculation>,
    grid: Grid<u8>,
}

#[derive(Clone, Debug)]
enum Calculation {
    Add(Vec<u64>),
    Multiply(Vec<u64>),
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let column_count = s.lines().next().unwrap_or("").split(" ").filter(|s| !s.trim().is_empty()).count();
        let mut calculation = vec![vec![]; column_count];
        let mut result = vec![];

        let lines = s.lines().collect::<Vec<_>>();

        for (i, line) in lines.iter().enumerate() {
            let splitted_line = line.split(" ").filter(|s| !s.trim().is_empty()).collect::<Vec<_>>();

            if i == lines.len() - 1 {
                for (j, value) in splitted_line.iter().enumerate() {
                    match *value {
                        "+" => result.push(Calculation::Add(calculation[j].clone())),
                        "*" => result.push(Calculation::Multiply(calculation[j].clone())),
                        _ => return Err(crate::aoc::Error::StringParse(format!("Unexpected operator: {}", value))),
                    }
                }
                continue;
            }
            for (j, value) in splitted_line.iter().enumerate() {
                calculation[j].push(value.parse::<u64>()?);
            }
        }

        let grid = Grid::parse_to_u8(s);



        Ok(Day {
            calculation: result,
            grid
        })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ", 4277556)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ", 3263827)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut total = 0;

        for calc in &self.calculation {
            match calc {
                Calculation::Add(values) => {
                    let sum: u64 = values.iter().sum();
                    total += sum;
                }
                Calculation::Multiply(values) => {
                    let product: u64 = values.iter().product();
                    total += product;
                }
            }
        }

        Ok(total)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let grid = &self.grid;
        let bottom = grid.height - 1;
        let mut right = grid.width;
        let mut total = 0;

        while right > 0 {
            let mut left = right - 1;
            while grid[(left, bottom)] == b' ' {
                left -= 1;
            }

            let plus = grid[(left, bottom)] == b'+';
            let iter = (left..right).map(|x| (0..bottom).fold(0, |num, y| acc(grid, num, x, y)));
            let acc: u64 = if plus { iter.sum() } else { iter.product() };


            if left != 0 {
                right = left - 1;
                total += acc;
            } else {
                total += acc;
                break;
            }
        }

        Ok(total)
    }
}

fn acc(grid: &Grid<u8>, number: u64, x: usize, y: usize) -> u64 {
    let digit = grid[(x, y)];
    if digit == b' ' { number } else { 10 * number + (digit - b'0') as u64 }
}
