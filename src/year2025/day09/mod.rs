use crate::utils::grid::{Coord, Grid};
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
pub struct Day {
    tiles: Vec<Coord>,
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| {
                let parts = line.split(',').collect::<Vec<_>>();
                let x: usize = parts[0].parse()?;
                let y: usize = parts[1].parse()?;
                Ok(Coord::from((x, y)))
            })
            .collect::<Result<Vec<Coord>, Self::Err>>()?;

        Ok(Day { tiles })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3", 50,)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3", 24)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        Ok(self
            .tiles
            .iter()
            .permutations(2)
            .fold(0, |mut acc, permutation| {
                let (tile, other) = (permutation[0], permutation[1]);
                let area: u64 = (((tile.0 as i64 - other.0 as i64).abs() + 1) * ((tile.1 as i64 - other.1 as i64).abs() + 1)) as u64;
                if area > acc {
                    acc = area;
                }

                acc
            }))
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut xs: BTreeSet<u64> = self.tiles.iter().map(|&(x, _)| x as u64).collect();
        xs.insert(0);
        xs.insert(u64::MAX);
        let shrink_x: BTreeMap<u64, i32> = xs.into_iter().enumerate().map(|(i, x)| (x, i as i32)).collect();

        let mut ys: BTreeSet<u64> = self.tiles.iter().map(|&(_, y)| y as u64).collect();
        ys.insert(0);
        ys.insert(u64::MAX);
        let shrink_y: BTreeMap<u64, i32> = ys.into_iter().enumerate().map(|(i, y)| (y, i as i32)).collect();

        let mut grid = Grid::<u8>::new(shrink_x.len(), shrink_y.len(), vec![b'X'; shrink_x.len() * shrink_y.len()]);

        for i in 0..self.tiles.len() {
            let (x1, y1) = self.tiles[i];
            let (x2, y2) = self.tiles[(i + 1) % self.tiles.len()];

            let x1 = shrink_x[&(x1 as u64)];
            let x2 = shrink_x[&(x2 as u64)];
            let y1 = shrink_y[&(y1 as u64)];
            let y2 = shrink_y[&(y2 as u64)];

            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    grid[Coord::from((x as usize, y as usize))] = b'#';
                }
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back(Coord::from((0, 0)));



        while let Some(point) = queue.pop_front() {
            for next in [(0, -1), (0, 1), (-1, 0), (1, 0)].map(|o| (point.0 as isize + o.0, point.1 as isize + o.1)) {
                if grid.in_bounds(next.0, next.1) {
                    let next = Coord::from((next.0 as usize, next.1 as usize));
                    if grid[next] == b'X' {
                        grid[next] = b'.';
                        queue.push_back(next);
                    }
                }
            }
        }

        let mut area = 0;

        for i in 0..self.tiles.len() {
            'outer: for j in i + 1..self.tiles.len() {
                let (x1, y1) = self.tiles[i];
                let (x2, y2) = self.tiles[j];

                let x1 = shrink_x[&(x1 as u64)];
                let x2 = shrink_x[&(x2 as u64)];
                let y1 = shrink_y[&(y1 as u64)];
                let y2 = shrink_y[&(y2 as u64)];

                for x in x1.min(x2)..=x1.max(x2) {
                    for y in y1.min(y2)..=y1.max(y2) {
                        if grid[Coord::from((x as usize, y as usize))] == b'.' {
                            continue 'outer;
                        }
                    }
                }

                let (x1, y1) = self.tiles[i];
                let (x2, y2) = self.tiles[j];
                let dx = x1.abs_diff(x2) + 1;
                let dy = y1.abs_diff(y2) + 1;
                area = area.max(dx * dy);
            }
        }

        Ok(area as u64)
    }
}
