use std::str::FromStr;
use crate::utils::grid::{CharIterator, Grid};

#[derive(Default, Clone, Debug)]
pub struct Day {
    grid: Grid<char>
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::<char>::from_raw_input(s)
        })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.", 13)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.", 43)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut iterator = CharIterator {
            grid: &self.grid,
            current_position: (0, 0),
            target: '@',
        };

        let mut count = 0;
        for coord in &mut iterator {
            let neighbours = self.grid.adjacent_coords_8(coord.0, coord.1);

            let amount_roll_neighbours = neighbours.iter().fold(0, |acc, &adj_coord| {
                if self.grid[adj_coord] == '@' {
                    acc + 1
                } else {
                    acc
                }
            });

            if amount_roll_neighbours < 4 {
                count += 1;
            }
        }

        Ok(count)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut remove_counter = 0;

        loop {
            let mut iterator = CharIterator {
                grid: &self.grid,
                current_position: (0, 0),
                target: '@',
            };
            let mut coords_to_remove = vec![];

            for coord in &mut iterator {
                let neighbours = self.grid.adjacent_coords_8(coord.0, coord.1);

                let amount_roll_neighbours = neighbours.iter().fold(0, |acc, &adj_coord| {
                    if self.grid[adj_coord] == '@' {
                        acc + 1
                    } else {
                        acc
                    }
                });

                if amount_roll_neighbours < 4 {
                    coords_to_remove.push(coord);
                }
            }

            if coords_to_remove.is_empty() {
                break;
            }

            for coord in &coords_to_remove {
                self.grid[*coord] = '.';
                remove_counter += 1;
            }
        }


        Ok(remove_counter)
    }
}