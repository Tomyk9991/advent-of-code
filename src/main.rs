use std::str::FromStr;
use crate::aoc::Day;

pub mod year2019;
pub mod year2022;
pub mod year2023;
pub mod utils;
pub mod aoc;

fn main() -> anyhow::Result<()> {
    type CurrentDay = year2023::day05::Day;

    let mut day = CurrentDay::from_str(include_str!("./year2023/day05/input.txt"))?;

    day.test_1()?;
    day.after_test_1();
    println!("Solution 1: {}", day.solution1()?);

    day.test_2()?;
    day.after_test_2();
    println!("Solution 2: {}", day.solution2()?);

    Ok(())
}