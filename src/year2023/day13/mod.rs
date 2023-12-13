use std::fmt::Debug;
use std::str::FromStr;
use itertools::Itertools;

use crate::aoc::Error;
use crate::utils::grid::Grid;

#[derive(Debug, Clone, Default)]
pub struct Day {
    grids: Vec<Grid<char>>,
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\r\n\r\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#", 405)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut horizontal = 0;
        let mut vertical = 0;

        for grid in &self.grids {
            let before = grid.find_all_reflections(0, 0, false);

            match before[0] {
                Orientation::Horizontal(x) => horizontal += x,
                Orientation::Vertical(x) => vertical += x,
            }
        }

        Ok(horizontal * 100 + vertical)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut horizonal = 0;
        let mut vertical = 0;

        for grid in self.grids.iter_mut() {

            let before = grid.find_all_reflections(0, 0, false);

            'outer: for y in 0..grid.height {
                for x in 0..grid.width {
                    let original = grid[(x, y)];
                    let mut index = None;

                    match original {
                        '#' => { grid[(x, y)] = '.'; index = Some((x, y)); }
                        '.' => { grid[(x, y)] = '#'; index = Some((x, y)); }
                        _ => { }
                    }

                    let after = grid.find_all_reflections(y, x, true);

                    if let Some(index) = index {
                        grid[index] = original;
                    }

                    if let Some(new) = after.clone().into_iter().collect_vec().into_iter().find(|x| !before.contains(x)) {
                        match new {
                            Orientation::Horizontal(h) => { horizonal += h; }
                            Orientation::Vertical(v) => { vertical += v; }
                        }
                        break 'outer;
                    }
                }
            }
        }

        Ok(horizonal * 100 + vertical)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Orientation {
    Horizontal(usize),
    Vertical(usize),
}


trait IDay {
    fn find_all_reflections(&self, r: usize, c: usize, need: bool) -> Vec<Orientation>;
}

impl IDay for Grid<char> {
    fn find_all_reflections(&self, row: usize, column: usize, need: bool) -> Vec<Orientation> {
        let mut result = vec![];
        for x in find_reflecting_rows(self, row, need) {
            result.push(Orientation::Horizontal(x))
        }

        for x in find_reflecting_rows(&self.transpose(), column, need) {
            result.push(Orientation::Vertical(x))
        }
        result
    }
}

fn find_reflecting_rows(grid: &Grid<char>, row: usize, need: bool) -> Vec<usize> {
    let mut result = vec![];
    let grid = grid.to_2d();

    for split_index in 1..(grid.len()) {
        let mut x = split_index - 1;
        let mut x_one = split_index;

        let mut good = true;
        let mut counts = 0;
        let mut used = false;


        loop {
            if grid[x] == grid[x_one] {
                counts += 1;
            } else {
                good = false;
            }

            if x == row || x_one == row {
                used = true;
            }

            if x < 1 || x == grid.len() - 1 || x_one < 1 || x_one == grid.len() - 1 {
                break;
            }

            x -= 1;
            x_one += 1;
        }
        if good && counts > 0 && (used || !need) {
            result.push(split_index);
        }
    }
    result
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let grids_str: Vec<&str> = src.split("\r\n\r\n")
            .collect();

        let mut grids = vec![];


        for grid_str in grids_str {
            let data = grid_str.lines()
                .flat_map(|line| { line.chars() })
                .collect::<Vec<_>>();

            let height = grid_str.lines().count();
            let width = if let Some(line) = grid_str.lines().next() {
                line.chars().count()
            } else {
                0
            };

            grids.push(Grid::new(width, height, data));
        }


        Ok(Self {
            grids
        })
    }
}