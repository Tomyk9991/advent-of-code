use std::{fs};

fn main() {
    let day_folder = "./src/year2023/";

    let num_days = fs::read_dir(day_folder)
        .expect(&format!("Unable to read directory {day_folder}"))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .count();

    let main_rs = format!(
        r#"use std::str::FromStr;
use crate::aoc::Day;

pub mod year2019;
pub mod year2022;
pub mod year2023;
pub mod utils;
pub mod aoc;

fn main() -> anyhow::Result<()> {{
    type CurrentDay = year2023::day{num_days:02}::Day;

    let mut day = CurrentDay::from_str(include_str!("./year2023/day{num_days:02}/input.txt"))?;

    day.test_1()?;
    day.after_test_1();
    println!("Solution 1: {{}}", day.solution1()?);

    day.test_2()?;
    day.after_test_2();
    println!("Solution 2: {{}}", day.solution2()?);

    Ok(())
}}"#);

    let main_rs_path = format!("./src/main.rs");
    let mod_rs_path = format!("{day_folder}mod.rs");

    let mut mod_rs: String = String::new();

    for i in 1..=num_days {
        mod_rs += &format!("pub mod day{i:02};\n");
    }


    fs::write(&mod_rs_path, mod_rs).expect("Unable to write to year2023/mod.rs");
    fs::write(&main_rs_path, main_rs).expect("Unable to write to main.rs");

    println!("cargo:rerun-if-changed=./src/year2023");
    println!("cargo:rerun-if-changed=build.rs");
}
