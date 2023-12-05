use std::collections::HashMap;
use std::str::FromStr;

use crate::aoc::Error;

#[derive(Default, Clone)]
pub struct Day {
    input: Vec<Path>
}

type Path = Vec<Direction>;
#[derive(Debug, Clone)]
enum Direction {
    Right(i32),
    Up(i32),
    Left(i32),
    Down(i32),
}

impl Direction {
    fn horizontal(&self) -> bool {
        matches!(self, Self::Left(_) | Self::Right(_))
    }

    fn positive(&self) -> bool {
        matches!(self, Direction::Right(_) | Direction::Up(_))
    }

    fn steps(&self) -> i32 {
        *match self {
            Direction::Right(a) => a,
            Direction::Up(a) => a,
            Direction::Left(a) => a,
            Direction::Down(a) => a,
        }
    }
}

fn move_direction(current_position: (i32, i32), direction: &Direction, hashmap: &mut HashMap<(i32, i32), [usize; 2]>, wire_index: usize) -> (i32, i32) {
    let horizontal = direction.horizontal();
    let positive = direction.positive();
    let amount_steps = direction.steps();

    let mut latest_position = (0, 0);

    for step in 1..=amount_steps {
        let pos = (
            current_position.0 + (if  horizontal { step } else { 0 } * if positive { 1 } else { -1 }),
            current_position.1 + (if !horizontal { step } else { 0 } * if positive { 1 } else { -1 }),
        );

        if let Some(wire_amount) = hashmap.get_mut(&pos) {
            wire_amount[wire_index] += 1;
        } else {
            let mut s = [0; 2];
            s[wire_index] += 1;
            hashmap.insert(pos, s);
        }

        latest_position = pos;
    }

    latest_position
}

fn steps_to_intersection(path: &Path, intersection: (i32, i32)) -> usize {
    let mut total_amount_steps = 0;

    let mut current_position = (0, 0);
    let mut latest_position = (0, 0);

    for direction in path {
        let horizontal = direction.horizontal();
        let positive = direction.positive();
        let amount_steps = direction.steps();


        for step in 1..=amount_steps {
            let position = (
                current_position.0 + (if  horizontal { step } else { 0 } * if positive { 1 } else { -1 }),
                current_position.1 + (if !horizontal { step } else { 0 } * if positive { 1 } else { -1 }),
            );

            total_amount_steps += 1;

            if position == intersection {
                return total_amount_steps;
            }

            latest_position = position;
        }

        current_position = latest_position;
    }

    total_amount_steps
}


impl crate::aoc::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("R8,U5,L5,D3\nU7,R6,D4,L4", 6),
            ("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83", 159),
            ("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 135)
        ]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("R8,U5,L5,D3\nU7,R6,D4,L4", 30),
            ("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83", 610),
            ("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 410)
        ]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut hashmap: HashMap<(i32, i32), [usize; 2]> = HashMap::new();

        for (wire_index, wire) in self.input.iter().enumerate() {
            let mut current_position: (i32, i32) = (0, 0);

            for direction in wire {
                current_position = move_direction(current_position, direction, &mut hashmap, wire_index);
            }
        }

        let min = hashmap.iter()
            .filter(|((_, _), amount_wires)| {
                amount_wires[0] >= 1 && amount_wires[1] >= 1
            })
            .map(|((x, y), _)| {
                x.abs() + y.abs()
            }).min().unwrap_or(0);


        Ok(min)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut hashmap: HashMap<(i32, i32), [usize; 2]> = HashMap::new();

        for (wire_index, wire) in self.input.iter().enumerate() {
            let mut current_position: (i32, i32) = (0, 0);

            for direction in wire {
                current_position = move_direction(current_position, direction, &mut hashmap, wire_index);
            }
        }

        let min = hashmap.iter()
            .filter(|((_, _), amount_wires)| {
                amount_wires[0] >= 1 && amount_wires[1] >= 1
            })
            .map(|((x, y), _)| {
                steps_to_intersection(&self.input[0], (*x, *y)) + steps_to_intersection(&self.input[1], (*x, *y))
            }).min().unwrap_or(0) as i32;

        Ok(min)
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            input: s.lines()
                .map(|line| line.split(',')
                    .map(to_direction)
                    .filter_map(|s| s.ok())
                    .collect::<Vec<_>>()
                ).collect::<Vec<_>>(),
        })
    }
}

fn to_direction(direction_str: &str) -> Result<Direction, Error> {
    let (direction, amount) = direction_str.split_at(1);
    match direction {
        "L" => Ok(Direction::Left(amount.parse::<i32>().map_err(|_| Error::StringParse(amount.to_string()))?)),
        "R" => Ok(Direction::Right(amount.parse::<i32>().map_err(|_| Error::StringParse(amount.to_string()))?)),
        "U" => Ok(Direction::Up(amount.parse::<i32>().map_err(|_| Error::StringParse(amount.to_string()))?)),
        "D" => Ok(Direction::Down(amount.parse::<i32>().map_err(|_| Error::StringParse(amount.to_string()))?)),
        s => Err(Error::StringParse(s.to_string())),
    }
}