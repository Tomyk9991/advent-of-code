use std::fmt::Debug;
use std::str::FromStr;

use crate::aoc::Error;

#[derive(Debug, Clone)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
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
    color: String,
}

#[derive(Debug, Clone, Default)]
pub struct Day {
    instructions: Vec<Instruction>,
}


impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)", 62)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)", 952408144115)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        // fuck flood fill
        let mut current_position: (isize, isize) = (0, 0);
        let polygons = self.instructions.iter().map(|instruction| {
            match instruction.direction {
                Direction::Up(d) => current_position.1 += d as isize,
                Direction::Down(d) => current_position.1 -= d as isize,
                Direction::Left(d) => current_position.0 -= d as isize,
                Direction::Right(d) => current_position.0 += d as isize
            };
            current_position
        }).collect::<Vec<_>>();

        let area = shoelace_formula_area(&polygons);
        let perimeter = shoelace_formula_perimeter(&polygons);

        Ok(area + (perimeter / 2 + 1))
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut current_position: (isize, isize) = (0, 0);

        let polygons = self.instructions.iter().map(|instruction| {
            let direction = hex_to_direction(&instruction.color);

            match direction {
                Direction::Up(d) => current_position.1 += d as isize,
                Direction::Down(d) => current_position.1 -= d as isize,
                Direction::Left(d) => current_position.0 -= d as isize,
                Direction::Right(d) => current_position.0 += d as isize
            };
            current_position
        }).collect::<Vec<_>>();

        let area = shoelace_formula_area(&polygons);
        let perimeter = shoelace_formula_perimeter(&polygons);

        Ok(area + (perimeter / 2 + 1))
    }
}

fn hex_to_direction(hex: &str) -> Direction {
    let mut hex = hex.to_string();
    let direction = hex.pop().unwrap();

    let length = usize::from_str_radix(&hex[1..6], 16).unwrap_or(0);
    match direction {
        '0' => Direction::Right(length),
        '1' => Direction::Down(length),
        '2' => Direction::Left(length),
        '3' => Direction::Up(length),
        _ => Direction::Up(0)
    }
}

fn shoelace_formula_area(polygon: &[(isize, isize)]) -> usize {
    let mut area = 0;

    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        area += (polygon[i].0 * polygon[j].1) - (polygon[j].0 * polygon[i].1);
    }

    area = area.abs() / 2;
    area as usize
}

fn shoelace_formula_perimeter(polygon: &[(isize, isize)]) -> usize {
    let mut perimeter = 0;

    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        perimeter += (polygon[i].0 - polygon[j].0).abs() + (polygon[i].1 - polygon[j].1).abs();
    }

    perimeter as usize
}


impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s.lines().filter_map(|line| {
            if let [direction, amount, color] = &line.split(' ').collect::<Vec<_>>()[..] {
                return Some(Instruction {
                    direction: Direction::from_str(direction, amount),
                    color: color.replace(['(', ')'], "").to_string(),
                });
            }

            None
        }).collect::<Vec<_>>();

        Ok(Self {
            instructions
        })
    }
}