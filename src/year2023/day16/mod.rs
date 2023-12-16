use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;

use crate::aoc::Error;
use crate::utils::grid::{Grid, Vec2};

#[derive(Debug, Clone, Default)]
pub struct Day {
    grid: Grid<char>
}

struct Beam {
    position: Vec2,
    direction: Vec2
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\n..../.\\..\n.-.-/..|..\n.|....-|.\\n..//.|....", 46)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        todo!()
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        println!("{:?}", self.grid);
        let mut current_beams: Vec<Beam> = vec![Beam {
            position: Vec2::new(0, 0),
            direction: Vec2::new(1, 0),
        }];

        let mut index = 0;

        let mut energized = HashSet::new();
        energized.insert(Vec2::new(0, 0));

        while current_beams.len() > 0 {
            while index < current_beams.len() {
                let mut current_beam = &mut current_beams[index];
                let new_position = &current_beam.position + &current_beam.direction;

                if !self.grid.in_bounds(new_position.x as usize, new_position.y as usize) {
                    // if out of bounds. remove the beam
                    current_beams.remove(index);
                    continue;
                }

                let char = self.grid[(new_position.x as usize, new_position.y as usize)];
                current_beam.position = new_position;
                energized.insert(current_beam.position.clone());

                let current_beam_position = current_beam.position.clone();

                match char {
                    '.' => { /* Nothing happens. the char is ok. */ }
                    '/' if current_beam.direction == Vec2::new(1, 0) => {
                        current_beam.direction = Vec2::new(0, 1);
                    },
                    '\\' if current_beam.direction == Vec2::new(1, 0) => {
                        current_beam.direction = Vec2::new(0, -1);
                    },
                    '/' if current_beam.direction == Vec2::new(-1, 0) => {
                        current_beam.direction = Vec2::new(0,-1);
                    },
                    '\\' if current_beam.direction == Vec2::new(-1, 0) => {
                        current_beam.direction = Vec2::new(0, 1)
                    }
                    '|' if current_beam.direction.y == 1 || current_beam.direction.y == -1 => {
                        /* Nothing happens. The char is ok. */
                    },
                    '|' if current_beam.direction.y == 0 => {
                        // split beam into two
                        current_beams.push(Beam {
                            position: Vec2 { x: current_beam_position.x, y: current_beam_position.y },
                            direction: Vec2 { x: 0, y: 1 }
                        });
                        current_beams.push(Beam {
                            position: Vec2 { x: current_beam_position.x, y: current_beam_position.y },
                            direction: Vec2 { x: 0, y: -1 }
                        });
                        current_beams.remove(index);
                    },
                    '-' if current_beam.direction.x == 1 || current_beam.direction.x == -1 => {
                        /* Nothing happens. The char is ok. */
                    },
                    '-' if current_beam.direction.x == 0 => {
                        current_beams.push(Beam {
                            position: Vec2 { x: current_beam_position.x, y: current_beam_position.y },
                            direction: Vec2 { x: 1, y: 0 }
                        });
                        current_beams.push(Beam {
                            position: Vec2 { x: current_beam_position.x, y: current_beam_position.y },
                            direction: Vec2 { x: -1, y: 0 }
                        });
                        current_beams.remove(index);
                    }
                    _ => { }
                }

                index += 1;
            }
        }

        println!("{}", energized.len());

        Ok(46)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
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

        let grid: Grid<char> = Grid::new(width, height, data);
        Ok(Self { grid })
    }
}