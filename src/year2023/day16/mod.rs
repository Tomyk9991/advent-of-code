use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::Deref;
use std::str::FromStr;

use crate::aoc::Error;
use crate::utils::grid::{Grid, Vec2};

#[derive(Debug, Clone, Default)]
pub struct Day {
    grid: Grid<char>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Beam {
    position: Vec2,
    direction: Vec2,
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#, 46)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#, 51)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        Ok(energize(&self.grid, Vec2::new(-1, 0), Vec2::new(1, 0)))
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut starting_positions = Vec::new();

        for x in 0..self.grid.width {
            starting_positions.push((Vec2 { x: x as isize, y: -1 }, Vec2::new(0, 1)));
            starting_positions.push((Vec2 { x: x as isize, y: self.grid.height as isize }, Vec2::new(0, -1)));
        }

        for y in 0..self.grid.height {
            starting_positions.push((Vec2 { x: -1, y: y as isize }, Vec2::new(1, 0)));
            starting_positions.push((Vec2 { x: self.grid.width as isize, y: y as isize }, Vec2::new(-1, 0)));
        }

        let s = starting_positions.iter().map(|(position, dir)| energize(&self.grid, position.clone(), dir.clone())).max();

        if let Some(s) = s {
            return Ok(s);
        }

        Ok(0)
    }
}

fn energize(grid: &Grid<char>, start_position: Vec2, direction: Vec2) -> usize {
    let mut current_beams: Vec<Beam> = vec![Beam {
        position: start_position,
        direction,
    }];

    let mut visited = HashSet::new();
    let mut energized = HashSet::new();

    while !current_beams.is_empty() {
        let current_beam = &mut current_beams[0];
        let new_position = &current_beam.position + &current_beam.direction;

        if !grid.in_bounds(new_position.x, new_position.y) {
            // if out of bounds. remove the beam
            current_beams.remove(0);
            continue;
        }

        if visited.get(current_beam.deref()).is_some() {
            current_beams.remove(0);
            continue;
        }

        visited.insert(current_beam.clone());

        let char = grid[(new_position.x as usize, new_position.y as usize)];
        current_beam.position = new_position;
        // energized[(current_beam.position.x as usize, current_beam.position.y as usize)] = '#';
        energized.insert(current_beam.position.clone());

        let current_beam_position = current_beam.position.clone();

        match char {
            '.' => { /* Nothing happens. the char is ok. */ }
            '/' => {
                current_beam.direction = match current_beam.direction {
                    Vec2 { x: 1, y: 0 } => Vec2::new(0, -1),
                    Vec2 { x: -1, y: 0 } => Vec2::new(0, 1),
                    Vec2 { x: 0, y: 1 } => Vec2::new(-1, 0),
                    Vec2 { x: 0, y: -1 } => Vec2::new(1, 0),
                    _ => unreachable!()
                }
            }
            '\\' => {
                current_beam.direction = match current_beam.direction {
                    Vec2 { x: 1, y: 0 } => Vec2::new(0, 1),
                    Vec2 { x: -1, y: 0 } => Vec2::new(0, -1),
                    Vec2 { x: 0, y: 1 } => Vec2::new(1, 0),
                    Vec2 { x: 0, y: -1 } => Vec2::new(-1, 0),
                    _ => unreachable!()
                }
            }
            '|' if current_beam.direction.y == 1 || current_beam.direction.y == -1 => {
                /* Nothing happens. The char is ok. */
            }
            '|' if current_beam.direction.y == 0 => {
                // split beam into two
                current_beams.push(Beam {
                    position: Vec2 { x: current_beam_position.x, y: current_beam_position.y },
                    direction: Vec2 { x: 0, y: 1 },
                });
                current_beams.push(Beam {
                    position: Vec2 { x: current_beam_position.x, y: current_beam_position.y },
                    direction: Vec2 { x: 0, y: -1 },
                });
                current_beams.remove(0);
            }
            '-' if current_beam.direction.x == 1 || current_beam.direction.x == -1 => {
                /* Nothing happens. The char is ok. */
            }
            '-' if current_beam.direction.x == 0 => {
                current_beams.push(Beam {
                    position: Vec2 { x: current_beam_position.x, y: current_beam_position.y },
                    direction: Vec2 { x: 1, y: 0 },
                });
                current_beams.push(Beam {
                    position: Vec2 { x: current_beam_position.x, y: current_beam_position.y },
                    direction: Vec2 { x: -1, y: 0 },
                });
                current_beams.remove(0);
            }
            _ => {}
        }
    }

    energized.len()
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
        Ok(Self { grid })
    }
}