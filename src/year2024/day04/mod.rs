use std::str::FromStr;
use crate::aoc::Error;
use crate::utils::grid::Grid;

#[derive(Default, Clone)]
pub struct Day {
    grid: Grid<char>
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let grid = s.lines()
            .map(|line| {
                width = line.len();
                line.chars().collect::<Vec<char>>()
            })
            .flatten()
            .collect::<Vec<char>>();

        let height = s.lines().count();
        Ok(Self {grid: Grid::new(width, height, grid) })
    }
}

impl crate::aoc::Day for Day {
    type Output = u32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX", 18)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX", 9)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let target_word = "XMAS".to_string();
        let reversed_target_word = target_word.chars().rev().collect::<String>();
        let directions = [
            (1, 0), (-1, 0), (0, 1), (0, -1),
            (1, 1), (-1, 1), (1, -1), (-1, -1)
        ];

        let count = self.grid.find_all(|&a| a == 'X')
            .iter()
            .flat_map(|&(x, y)| directions.iter().map(move |&(dx, dy)| (x, y, dx, dy)))
            .filter(|&(x, y, dx, dy)| {
                let word: String = (0..target_word.len())
                    .map(|i| self.grid.get(((x as i32 + i as i32 * dx) as usize, (y as i32 + i as i32 * dy) as usize)))
                    .take_while(|&c| c.is_some())
                    .filter_map(|c| c)
                    .collect();
                word == target_word || word == reversed_target_word
            })
            .count();

        Ok(count as u32)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut count = 0;
        let target_word = "MAS".to_string();
        let reversed_target_word = target_word.chars().rev().collect::<String>();
        let x_indices = self.grid.find_all(|a| *a == 'A');

        for x_index in x_indices {
            if x_index.0 == 0 || x_index.1 == 0 || x_index.0 == self.grid.width - 1 || x_index.1 == self.grid.height - 1 {
                continue;
            }

            let mut word_1 = String::new();


            let top_left_position = ((x_index.0 as i32 - 1).max(0) as usize, (x_index.1 as i32 - 1).max(0) as usize);
            let bottom_right_position = ((x_index.0 as i32 + 1).min(self.grid.width as i32 - 1) as usize, (x_index.1 as i32 + 1).min(self.grid.height as i32 - 1) as usize);

            word_1.push(self.grid[top_left_position]);
            word_1.push(self.grid[(x_index.0, x_index.1)]);
            word_1.push(self.grid[bottom_right_position]);

            let mut word_2 = String::new();
            let top_right_position = ((x_index.0 as i32 + 1).min(self.grid.width as i32 - 1) as usize, (x_index.1 as i32 - 1).max(0) as usize);
            let bottom_left_position = ((x_index.0 as i32 - 1).max(0) as usize, (x_index.1 as i32 + 1).min(self.grid.height as i32 - 1) as usize);

            word_2.push(self.grid[bottom_left_position]);
            word_2.push(self.grid[(x_index.0, x_index.1)]);
            word_2.push(self.grid[top_right_position]);

            if (word_1 == target_word || word_1 == reversed_target_word) && (word_2 == target_word || word_2 == reversed_target_word) {
                count += 1;
            }
        }


        Ok(count)
    }
}