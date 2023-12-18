use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::str::FromStr;

use crate::aoc::Error;
use crate::utils::grid::{Grid, Vec2};

#[derive(Debug, Clone)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize)
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up(0)
    }
}

impl Direction {
    fn from_str(direction: &str, direction_step: &str) -> Self {
        let direction_step = direction_step.parse::<usize>().unwrap_or(0);

        match direction {
            "U" => Direction::Up(direction_step),
            "D" => Direction::Down(direction_step),
            "L" => Direction::Left(direction_step),
            "R" => Direction::Right(direction_step),
            _ => Direction::Up(0)
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Instruction {
    direction: Direction,
    color: String
}

#[derive(Debug, Clone, Default)]
pub struct Day {
    instructions: Vec<Instruction>
}


impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)", 62)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        todo!()
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut bounds_x = (isize::MAX, isize::MIN);
        let mut bounds_y = (isize::MAX, isize::MIN);

        let mut current_position = Vec2 { x: 0, y: 0 };
        let mut edges = vec![];

        for instruction in &self.instructions {
            edges.push(current_position.clone());

            let mut min = 0;
            let mut max = 0;
            let mut x_dir = true;

            match instruction.direction {
                Direction::Up(d) => {
                    min = current_position.y;
                    current_position.y += d as isize;
                    max = current_position.y;
                    x_dir = false;

                },
                Direction::Down(d) => {
                    max = current_position.y;
                    current_position.y -= d as isize;
                    min = current_position.y;
                    x_dir = false;
                },
                Direction::Left(d) => {
                    max = current_position.x;
                    current_position.x -= d as isize;
                    min = current_position.x;
                },
                Direction::Right(d) => {
                    min = current_position.x;
                    current_position.x += d as isize;
                    max = current_position.x;
                }
            }

            for value in min..max {
                if x_dir {
                    edges.push(Vec2 { x: value, y: current_position.y })
                } else {
                    edges.push(Vec2 { x: current_position.x, y: value })
                }
            }

            if current_position.x < bounds_x.0 {
                bounds_x.0 = current_position.x;
            }

            if current_position.x > bounds_x.1 {
                bounds_x.1 = current_position.x;
            }

            if current_position.y < bounds_y.0 {
                bounds_y.0 = current_position.y;
            }

            if current_position.y > bounds_y.1 {
                bounds_y.1 = current_position.y;
            }
        }

        let mut grid = Grid::new(
            (bounds_x.1 - bounds_x.0) as usize + 1,
            (bounds_y.1 - bounds_y.0) as usize + 1,
            vec!['.'; (((bounds_x.1 - bounds_x.0) as usize + 1) * ((bounds_y.1 - bounds_y.0) as usize + 1))]
        );

        println!("{:?} {:?}", bounds_x, bounds_y);
        println!("{:?} {:?}", grid.width, grid.height);

        for edge in &edges {
            let x = if bounds_x.0 >= 0 { edge.x + bounds_x.0 } else { edge.x + (bounds_x.1 - bounds_x.0) } as usize;
            let y = if bounds_y.0 >= 0 { edge.y + bounds_y.0 } else { edge.y + (bounds_y.1 - bounds_y.0) } as usize;
            println!("{edge:?} => x: {x} y: {y}");
            grid[(x, y)] = '#';
        }


        let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
        let mut hashset = HashSet::new();
        let mut counter = 0;

        queue.push_back((1, 1));
        hashset.insert((1, 1));

        while let Some((x, y)) = queue.pop_front() {
            if grid.inside(x, y) {
                grid[(x as usize, y as usize)] = '#';
                counter += 1;

                if !hashset.contains(&(x + 1, y)) {
                    queue.push_back((x + 1, y));
                    hashset.insert((x + 1, y));
                }

                if !hashset.contains(&(x - 1, y)) {
                    queue.push_back((x - 1, y));
                    hashset.insert((x - 1, y));
                }

                if !hashset.contains(&(x, y + 1)) {
                    queue.push_back((x, y + 1));
                    hashset.insert((x, y + 1));
                }

                if !hashset.contains(&(x, y - 1)) {
                    queue.push_back((x, y - 1));
                    hashset.insert((x, y - 1));
                }
            }
        }

        Ok(counter)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        todo!()
    }
}

trait Inside {
    fn inside(&self, x: isize, y: isize) -> bool;
}

impl Inside for Grid<char> {
    fn inside(&self, x: isize, y: isize) -> bool {
        // check if you can go from current position to left and find #
        // ...
        // check if you can go from current position to bottom and find #
        if !self.in_bounds(x, y) {
            return false;
        }

        if self[(x as usize, y as usize)] == '#' {
            return true;
        }

        let mut found_left = false;
        for new_x in (0..=x - 1).rev() {
            if !self.in_bounds(new_x, y) {
                break;
            }

            if self[(new_x as usize, y as usize)] == '#' {
                found_left = true;
            }
        }

        let mut found_right = false;
        for new_x in x + 1..self.width as isize {
            if !self.in_bounds(new_x, y) {
                break;
            }

            if self[(new_x as usize, y as usize)] == '#' {
                found_right = true;
            }
        }

        let mut found_up = false;
        for new_y in (0..=y - 1).rev() {
            if !self.in_bounds(x, new_y) {
                break;
            }

            if self[(x as usize, new_y as usize)] == '#' {
                found_up = true;
            }
        }

        let mut found_down = false;
        for new_y in y + 1..self.height as isize {
            if !self.in_bounds(x, new_y) {
                break;
            }

            if self[(x as usize, new_y as usize)] == '#' {
                found_down = true;
            }
        }


        return found_left && found_right && found_up && found_down;
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s.lines().filter_map(|line| {
            if let [direction, amount, color] = &line.split(' ').collect::<Vec<_>>()[..] {
                return Some(Instruction {
                    direction: Direction::from_str(direction, amount),
                    color: color.replace('(', "").replace(')', "").to_string(),
                });
            }

            return None
        }).collect::<Vec<_>>();

        Ok(Self {
            instructions
        })
    }
}