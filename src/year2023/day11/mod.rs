use std::collections::HashSet;
use std::str::FromStr;
use itertools::Itertools;

use crate::aoc::Error;
use crate::utils::grid::Grid;


#[derive(Debug, Clone, Default)]
pub struct Day {
    grid: Grid<char>,
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....", 374)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        todo!()
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        todo!()
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        todo!()
    }
}

trait Empty {
    fn row(&self) -> Vec<usize>;
    fn column(&self) -> Vec<usize>;

    fn expand_row(&mut self, row: usize);
    fn expand_column(&mut self, column: usize);
}

impl Empty for Grid<char> {
    fn row(&self) -> Vec<usize> {
        let mut rows = vec![];
        for y in 0..self.height {
            let mut found_galaxy = false;

            for x in 0..self.width {
                if self[(x, y)] == '#' {
                    found_galaxy = true;
                    break;
                }
            }

            if !found_galaxy {
                rows.push(y);
            }
        }

        return rows;
    }

    fn column(&self) -> Vec<usize> {
        let mut columns = vec![];
        for x in 0..self.width {
            let mut found_galaxy = false;

            for y in 0..self.height {
                if self[(x, y)] == '#' {
                    found_galaxy = true;
                    break;
                }
            }

            if !found_galaxy {
                columns.push(x);
            }
        }

        return columns;
    }

    fn expand_row(&mut self, row: usize) {
        todo!()
    }

    fn expand_column(&mut self, column: usize) {
        todo!()
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.lines()
            .flat_map(|line| { line.chars() })
            .collect::<Vec<_>>();

        let height = s.lines().count();
        let width = if let Some(line) = s.lines().next() {
            line.chars().count()
        } else {
            0
        };

        println!("width: {} height: {} data: {}", width, height, data.len());

        let mut grid: Grid<char> = Grid::new(width, height, data);
        let empty_rows = grid.row();
        let empty_columns = grid.column();


        println!("{:?}", empty_rows);
        println!("{:?}", empty_columns);

        // empty_rows.iter().for_each(|row| grid.expand_row(*row));
        // empty_columns.iter().for_each(|column| grid.expand_column(*column));

        return Ok(Self {
            grid
        })
    }
}