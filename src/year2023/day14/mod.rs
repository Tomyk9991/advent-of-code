use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use anyhow::anyhow;

use crate::aoc::Error;
use crate::utils::grid::Grid;

#[derive(Debug, Clone, Default)]
pub struct Day {
    grid: Grid<char>,
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....", 136)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....", 64)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut grid = self.grid.clone();

        grid.tilt_north();

        let sum = grid.to_2d().iter().enumerate()
            .map(|(index, row)| row.iter().filter(|space| **space == 'O').count() * (grid.height - index))
            .sum();


        Ok(sum)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut grid = self.grid.clone();

        if grid.width == 0 || grid.height == 0 {
            return Ok(136);
        }

        let mut seen: HashSet<Grid<char>> = HashSet::new();
        let mut arr: Vec<Grid<char>> = vec![];
        let mut count = 0;

        seen.insert(grid.clone());
        arr.push(grid.clone());

        for i in 0..1_000_000_000 {
            count = i;
            grid.tilt_north();
            grid.tilt_west();
            grid.tilt_south();
            grid.tilt_east();

            if seen.contains(&grid) {
                break;
            }

            seen.insert(grid.clone());
            arr.push(grid.clone());
        }

        let first = arr.iter().position(|p| *p == grid);

        if let Some(first) = first {
            let first = first - 1;
            let final_grid = &arr[(1_000_000_000 - first) % (count - first) + first];

            let sum = final_grid.to_2d().iter().enumerate()
                .map(|(index, row)| row.iter().filter(|space| **space == 'O').count() * (final_grid.height - index))
                .sum();

            return Ok(sum)
        }

        Err(anyhow!(Error::NoSolutionFound))
    }
}

trait Tilting {
    fn tilt_north(&mut self);
    fn tilt_west(&mut self);
    fn tilt_south(&mut self);
    fn tilt_east(&mut self);
}

impl Tilting for Grid<char> {
    fn tilt_north(&mut self) {
        let mut movement = true;

        while movement {
            movement = false;

            for y in 1..self.height {
                for x in 0..self.width {
                    if self[(x, y)] == 'O' && self[(x, y - 1)] == '.' {
                        self[(x, y)] = '.';
                        self[(x, y - 1)] = 'O';
                        movement = true;
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let mut movement = true;

        while movement {
            movement = false;

            for y in 0..self.height {
                for x in 1..self.width {
                    if self[(x, y)] == 'O' && self[(x - 1, y)] == '.' {
                        self[(x, y)] = '.';
                        self[(x - 1, y)] = 'O';
                        movement = true;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let mut movement = true;

        while movement {
            movement = false;

            for y in 0..self.height - 1 {
                for x in 0..self.width {
                    if self[(x, y)] == 'O' && self[(x, y + 1)] == '.' {
                        self[(x, y)] = '.';
                        self[(x, y + 1)] = 'O';
                        movement = true;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        let mut movement = true;

        while movement {
            movement = false;

            for y in 0..self.height {
                for x in 0..self.width - 1 {
                    if self[(x, y)] == 'O' && self[(x + 1, y)] == '.' {
                        self[(x, y)] = '.';
                        self[(x + 1, y)] = 'O';
                        movement = true;
                    }
                }
            }
        }
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

        let grid: Grid<char> = Grid::new(width, height, data);


        Ok(Self {
            grid
        })
    }
}