use std::collections::HashSet;
use std::str::FromStr;
use itertools::Itertools;
use crate::utils::grid::Grid;

#[derive(Default, Clone)]
pub struct Day {
    grid: Grid<Vec<char>>
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let grid = s.lines()
            .map(|line| {
                width = line.len();
                line.chars().map(|a| vec![a]).collect::<Vec<Vec<char>>>()
            })
            .flatten()
            .collect::<Vec<Vec<char>>>();

        let height = s.lines().count();
        Ok(Self {
            grid: Grid::new(width, height, grid)
        })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#, 14)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#, 34)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        // unique letters and digits
        let unique_letters_and_digits = self.grid.into_iter()
            .flatten()
            .filter(|c| c.is_alphanumeric())
            .unique()
            .collect::<Vec<&char>>();

        let mut unique_positions = HashSet::new();
        for unique_letter in &unique_letters_and_digits {
            let all_letter_positions = self.grid.find_all(|c| c[0] == **unique_letter);
            let combinations = all_letter_positions.iter().combinations(2);


            for combination in combinations {
                if let [(pos1_x, pos1_y), (pos2_x, pos2_y)] = &combination[..] {
                    let difference_x = *pos2_x as i32 - *pos1_x as i32;
                    let difference_y = *pos2_y as i32 - *pos1_y as i32;
                    let new_pos_1 = ((*pos1_x as i32) - difference_x, (*pos1_y as i32) - difference_y);
                    let new_pos_2 = ((*pos2_x as i32) + difference_x, (*pos2_y as i32) + difference_y);

                    if self.grid.in_bounds(new_pos_1.0 as isize, new_pos_1.1 as isize) {
                        unique_positions.insert(new_pos_1);
                    }

                    if self.grid.in_bounds(new_pos_2.0 as isize, new_pos_2.1 as isize) {
                        unique_positions.insert(new_pos_2);
                    }
                }
            }
        }

        Ok(unique_positions.len() as u64)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        // unique letters and digits
        let unique_letters_and_digits = self.grid.into_iter()
            .flatten()
            .filter(|c| c.is_alphanumeric())
            .unique()
            .collect::<Vec<&char>>();

        let mut unique_positions = HashSet::new();
        for unique_letter in &unique_letters_and_digits {
            let all_letter_positions = self.grid.find_all(|c| c[0] == **unique_letter);
            let combinations = all_letter_positions.iter().combinations(2);

            for combination in combinations {
                if let [(pos1_x, pos1_y), (pos2_x, pos2_y)] = &combination[..] {
                    unique_positions.insert((*pos1_x as i32, *pos1_y as i32));
                    unique_positions.insert((*pos2_x as i32, *pos2_y as i32));

                    let difference_x = *pos2_x as i32 - *pos1_x as i32;
                    let difference_y = *pos2_y as i32 - *pos1_y as i32;

                    let mut new_pos_1 = ((*pos1_x as i32) - difference_x, (*pos1_y as i32) - difference_y);

                    while self.grid.in_bounds(new_pos_1.0 as isize, new_pos_1.1 as isize) {
                        unique_positions.insert(new_pos_1);
                        new_pos_1 = (new_pos_1.0 - difference_x, new_pos_1.1 - difference_y);
                    }

                    let mut new_pos_2 = ((*pos2_x as i32) + difference_x, (*pos2_y as i32) + difference_y);

                    while self.grid.in_bounds(new_pos_2.0 as isize, new_pos_2.1 as isize) {
                        unique_positions.insert(new_pos_2);
                        new_pos_2 = (new_pos_2.0 + difference_x, new_pos_2.1 + difference_y);
                    }
                }
            }
        }

        Ok(unique_positions.len() as u64)
    }
}