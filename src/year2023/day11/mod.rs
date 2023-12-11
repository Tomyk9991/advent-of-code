use std::fmt::Debug;
use std::str::FromStr;
use itertools::Itertools;
use crate::aoc::Error;
use crate::utils::grid::{CharIterator, Coord, Distance, Grid};


#[derive(Debug, Clone, Default)]
pub struct Day {
    galaxy_positions: Vec<Coord>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....", 374)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        // vec![("...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....", 82000210)]
        vec![]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let expansion = 1;
        let mut galaxy_positions = self.galaxy_positions.clone();

        for galaxy_position in galaxy_positions.iter_mut() {
            galaxy_position.0 += expansion * (self.empty_columns.iter().filter(|column| galaxy_position.0 > **column).count());
            galaxy_position.1 += expansion * (self.empty_rows.iter().filter(|row| galaxy_position.1 > **row).count());
        }

        let pairs = galaxy_positions.into_iter().combinations(2).collect::<Vec<_>>();
        let mut sum = 0;

        for pairs in &pairs {
            if let [p1, p2] = pairs[..] {
                sum += p1.manhattan_distance(&p2);
            }
        }

        Ok(sum)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let expansion = 1000000 - 1;
        let mut galaxy_positions = self.galaxy_positions.clone();

        for galaxy_position in galaxy_positions.iter_mut() {
            galaxy_position.0 += expansion * self.empty_columns.iter().filter(|column| galaxy_position.0 > **column).count();
            galaxy_position.1 += expansion * self.empty_rows.iter().filter(|row| galaxy_position.1 > **row).count();
        }

        let pairs = galaxy_positions.into_iter().combinations(2).collect::<Vec<_>>();
        let mut sum = 0;

        for pairs in &pairs {
            if let [p1, p2] = pairs[..] {
                sum += p1.manhattan_distance(&p2);
            }
        }

        Ok(sum)
    }
}

trait Empty {
    fn row(&self) -> Vec<usize>;
    fn column(&self) -> Vec<usize>;
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

        rows
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

        columns
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

        let empty_rows = grid.row();
        let empty_columns = grid.column();

        let points = CharIterator {
            grid: &grid,
            current_position: (0, 0),
            target: '#',
        }.collect::<Vec<_>>();


        Ok(Self {
            galaxy_positions: points,
            empty_rows,
            empty_columns,
        })
    }
}