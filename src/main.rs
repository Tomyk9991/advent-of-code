use std::str::FromStr;
use std::time::Instant;
use crate::aoc::Day;

pub mod year2025;
pub mod utils;
pub mod aoc;

fn main() -> anyhow::Result<()> {
    type CurrentDay = year2025::day04::Day;

    // running day in year
    let mut day = CurrentDay::from_str(include_str!("./year2025/day04/input.txt"))?;

    println!("Running Year 2025 Day 04");

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