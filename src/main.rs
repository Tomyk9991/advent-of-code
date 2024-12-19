use std::str::FromStr;
use std::time::Instant;
use crate::aoc::Day;

pub mod year2024;
pub mod utils;
pub mod aoc;

fn main() -> anyhow::Result<()> {
    type CurrentDay = year2024::day19::Day;

    let mut day = CurrentDay::from_str(include_str!("./year2024/day19/input.txt"))?;

    day.test_1()?;
    day.after_test_1();
    println!("Test 1 passed");
    let time = Instant::now();
    println!("Solution 1: {:<20} took ~{}ms", day.solution1()?, (Instant::now() - time).as_millis());

    day.test_2()?;
    day.after_test_2();
    println!("Test 2 passed");
    let time = Instant::now();
    println!("Solution 2: {:<20} took ~{}ms", day.solution2()?, (Instant::now() - time).as_millis());

    Ok(())
}