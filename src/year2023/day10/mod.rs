use std::collections::HashSet;
use std::str::FromStr;

use crate::aoc::Error;
use crate::utils::grid::Grid;

#[derive(Debug, Default)]
struct Direction {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

impl std::ops::AddAssign for Direction {
    fn add_assign(&mut self, rhs: Self) {
        self.up += rhs.up;
        self.down += rhs.down;
        self.left += rhs.left;
        self.right += rhs.right;
    }
}


impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '|' => Direction { up: 1, down: 1, left: 0, right: 0 },
            '-' => Direction { up: 0, down: 0, left: 1, right: 1 },
            'L' => Direction { up: 1, down: 0, left: 0, right: 1 },
            'J' => Direction { up: 1, down: 0, left: 1, right: 0 },
            '7' => Direction { up: 0, down: 1, left: 1, right: 0 },
            'F' => Direction { up: 0, down: 1, left: 0, right: 1 },
            'S' => Direction { up: 1, down: 1, left: 1, right: 1 },
            _ => Direction { up: 0, down: 0, left: 0, right: 0 },
        }
    }
}

impl Direction {
    pub fn horizontal(&self) -> bool {
        self.left % 2 == 1 && self.right % 2 == 1
    }

    pub fn vertical(&self) -> bool {
        self.up % 2 == 1 && self.down % 2 == 1
    }
}

#[derive(Debug, Clone, Default)]
pub struct Day {
    grid: Grid<char>,
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![
            (".....\n.S-7.\n.|.|.\n.L-J.\n.....", 4),
            ("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...", 8),
        ]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........", 4),
            (".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...", 8),
            ("FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L", 10)
        ]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let start = self.grid.find(|a| *a == 'S').ok_or(Error::NoSolutionFound)?;
        Ok(self.grid.propagate_loop(start).len() / 2)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let start = self.grid.find(|a| *a == 'S').ok_or(Error::NoSolutionFound)?;
        let looop = self.grid.propagate_loop(start);

        let hash_set:HashSet<(usize, usize)> = looop.into_iter().collect();
        let mut enclosed = 0;

        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                if self.grid.enclosed(&hash_set, (x, y)) {
                    enclosed += 1;
                }
            }
        }

        Ok(enclosed)
    }
}

trait Part1And2 {
    fn propagate_loop(&self, start: (usize, usize)) -> Vec<(usize, usize)>;
    fn step(&self, current: (usize, usize), previous: (usize, usize)) -> (usize, usize);
    fn enclosed(&self, hashset: &HashSet<(usize, usize)>, position: (usize, usize)) -> bool;
}


impl Part1And2 for Grid<char> {
    fn propagate_loop(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut current = start;
        let mut previous = start;
        let mut positions = vec![];

        loop {
            let next = self.step(current, previous);
            previous = current;
            current = next;
            positions.push(current);

            if current == start {
                break;
            }
        }

        positions
    }

    fn step(&self, current: (usize, usize), previous: (usize, usize)) -> (usize, usize) {
        let pipe = self[current];

        if pipe == 'S' {
            if let '-' | 'J' | '7' = self[(current.0 + 1, current.1)] {
                return (current.0 + 1, current.1);
            } else if let '|' | 'L' | 'J' = self[(current.0, current.1 + 1)] {
                return (current.0, current.1 + 1);
            } else if let '-' | 'L' | 'F' = self[(current.0.wrapping_sub(1), current.1)] {
                return (current.0 - 1, current.1);
            } else if let '|' | '7' | 'F' = self[(current.0, current.1.wrapping_sub(1))] {
                return (current.0, current.1 - 1);
            }
        }

        let [p1, p2] = match pipe {
            '|' => [
                (current.0, current.1.wrapping_sub(1)),
                (current.0, current.1 + 1),
            ],
            '-' => [
                (current.0.wrapping_sub(1), current.1),
                (current.0 + 1, current.1),
            ],
            'L' => [
                (current.0, current.1.wrapping_sub(1)),
                (current.0 + 1, current.1),
            ],
            'J' => [
                (current.0, current.1.wrapping_sub(1)),
                (current.0.wrapping_sub(1), current.1),
            ],
            '7' => [
                (current.0.wrapping_sub(1), current.1),
                (current.0, current.1 + 1),
            ],
            'F' => [
                (current.0 + 1, current.1),
                (current.0, current.1 + 1),
            ],
            _ => panic!("REE")
        };

        if p1 == previous {
            p2
        } else {
            p1
        }
    }

    fn enclosed(&self, hashset: &HashSet<(usize, usize)>, position: (usize, usize)) -> bool {
        if hashset.contains(&position) {
            return false;
        }

        let mut direction_x = Direction::default();
        for x in 0..position.0 {
            if hashset.contains(&(x, position.1)) {
                let pipe = self[(x, position.1)];
                direction_x += Direction::from(pipe);
            }
        }

        let mut direction_y = Direction::default();

        for y in 0..position.1 {
            if hashset.contains(&(position.0, y)) {
                let pipe = self[(position.0, y)];
                direction_y += Direction::from(pipe);
            }
        }

        direction_x.vertical() && direction_y.horizontal()
    }
}


impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.lines()
            .flat_map(|line| { line.chars() })
            .collect::<Vec<_>>();

        let height = data.len();
        let width = if let Some(line) = s.lines().next() {
            line.chars().count()
        } else {
            0
        };

        Ok(Self { grid: Grid::new(width, height, data) })
    }
}