use std::collections::HashSet;
use std::str::FromStr;
use itertools::Itertools;
use crate::utils::grid::Grid;

#[derive(Default, Clone, PartialEq)]
enum Tile {
    #[default]
    Empty,
    Obstacle,
    Guard { x_dir: i32, y_dir: i32 }
}


impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Obstacle,
            '^' => Tile::Guard { x_dir: 0, y_dir: -1 },
            a => panic!("Invalid tile: {}", a)
        }
    }
}

#[derive(Default, Clone)]
pub struct Day {
    grid: Grid<Tile>
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let grid = s.lines()
            .map(|line| {
                width = line.len();
                line.chars().map(|c| c.into()).collect::<Vec<Tile>>()
            })
            .flatten()
            .collect::<Vec<Tile>>();

        let height = s.lines().count();
        Ok(Self {grid: Grid::new(width, height, grid) })
    }
}

impl crate::aoc::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#, 41)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#, 6)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        match self.calculate_solution_1() {
            Some(a) => {
                Ok(a.iter().unique_by(|pos| (pos.x, pos.y)).count() as i32)
            },
            None => Err(crate::aoc::Error::NoSolutionFound)?
        }
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let original_path = self.calculate_solution_1().ok_or(crate::aoc::Error::NoSolutionFound)?;
        let start_pos = original_path[0].clone();
        let mut valid_positions = 0;

        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                if self.grid[(x, y)] == Tile::Empty && (x as i32, y as i32) != (start_pos.x, start_pos.y) {

                    self.grid[(x, y)] = Tile::Obstacle;

                    if let None = self.calculate_solution_1() {
                        valid_positions += 1;
                    }

                    self.grid[(x, y)] = Tile::Empty;
                }
            }
        }

        Ok(valid_positions)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Solution {
    x: i32,
    y: i32,
    x_dir: i32,
    y_dir: i32,
}

impl Day {
    fn calculate_solution_1(&self) -> Option<Vec<Solution>> {
        let (start_x, start_y) = self.grid.find(|tile| matches!(tile, Tile::Guard { .. }))?;
        let (mut x, mut y) = (start_x as i32, start_y as i32);
        let (mut x_dir, mut y_dir) = match self.grid[(start_x, start_y)] {
            Tile::Guard { x_dir, y_dir } => (x_dir, y_dir),
            _ => return None,
        };

        let mut distinct_positions = Vec::new();
        let mut seen = HashSet::new();

        loop {
            if !seen.insert((x, y, x_dir, y_dir)) {
                // Wir haben eine Schleife gefunden, beenden wir hier
                return None;
            }
            distinct_positions.push(Solution { x, y, x_dir, y_dir });

            let (next_x, next_y) = (x + x_dir, y + y_dir);
            if next_x < 0 || next_y < 0 || next_x >= self.grid.width() as i32 || next_y >= self.grid.height() as i32 {
                break;
            }

            match self.grid[(next_x as usize, next_y as usize)] {
                Tile::Empty | Tile::Guard { .. } => {
                    x = next_x;
                    y = next_y;
                }
                Tile::Obstacle => {
                    std::mem::swap(&mut x_dir, &mut y_dir);
                    x_dir = -x_dir;
                }
            }
        }

        Some(distinct_positions)
    }
}