use std::fs;


pub struct Day1;

impl crate::year2022::Day for Day1 {
    fn date(&self) -> (i32, i32) { (1, 2022) }

    fn run(&self) {
        let input = fs::read_to_string("src/year_2022/day1/input.txt")
            .unwrap();

        let mut lines: Vec<u32> = input
            .split("\n\n")
            .map(|line| {
                line.split('\n')
                    .flat_map(|num| num.parse::<u32>())
                    .sum()
            })
            .collect::<Vec<u32>>();

        lines.sort_by(|a, b| b.cmp(a));

        println!("{}", lines[0]);
        println!("{}", lines.iter().take(3).sum::<u32>());
    }
}