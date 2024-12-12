use std::str::FromStr;
use itertools::Itertools;
use crate::utils::grid::{Grid};

#[derive(Default, Clone)]
pub struct Day {
    grid: Grid<char>,
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let grid = s.lines()
            .map(|line| {
                width = line.len();
                line.chars().collect::<Vec<char>>()
            })
            .flatten()
            .collect::<Vec<char>>();

        let height = s.lines().count();
        Ok(Self { grid: Grid::new(width, height, grid) })
    }
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#, 1930)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#, 1206), (r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#, 368)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let segments = self.grid.floor_fill(|current_segment_value, current_value| current_segment_value == current_value);
        let sum = segments.iter().map(|s| (s.len(), perimeter(s))).fold(0, |acc, (a, b)| acc + a * b);
        Ok(sum)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let segments = self.grid.floor_fill(|current_segment_value, current_value| current_segment_value == current_value);
        let sum = segments.iter().map(|s| (s.len(), amount_sides(&s.iter().map(|a| (a.0 as isize, a.1 as isize)).collect_vec()))).fold(0, |acc, (a, b)| acc + a * b);
        Ok(sum)
    }
}

fn amount_sides(segment: &Vec<(isize, isize)>) -> usize {
    // V - E + F = 2 mit F = 2
    // V - E + 2 = 2
    // V - E = 0
    // V = E
    let mut corner_count = 0;

    for coord in segment {
        let left_neighbour = (coord.0 - 1, coord.1);
        let right_neighbour = (coord.0 + 1, coord.1);
        let top_neighbour = (coord.0, coord.1 - 1);
        let bottom_neighbour = (coord.0, coord.1 + 1);

        let top_left_neighbour = (coord.0 - 1, coord.1 - 1);
        let top_right_neighbour = (coord.0 + 1, coord.1 - 1);
        let bottom_left_neighbour = (coord.0 - 1, coord.1 + 1);
        let bottom_right_neighbour = (coord.0 + 1, coord.1 + 1);

        for (diagonal_pos, component_1, component_2) in [
            (top_left_neighbour, top_neighbour, left_neighbour),
            (top_right_neighbour, top_neighbour, right_neighbour),
            (bottom_left_neighbour, bottom_neighbour, left_neighbour),
            (bottom_right_neighbour, bottom_neighbour, right_neighbour),
        ].iter() {
            if !segment.contains(&component_1) && !segment.contains(&component_2) && !segment.contains(&diagonal_pos) {
                corner_count += 1;
            }

            if segment.contains(&component_1) && segment.contains(&component_2) && !segment.contains(&diagonal_pos) {
                corner_count += 1;
            }
        }


        // look for pattern A, x   OR    x, A
        //                  x, A   OR    A, x
        if !segment.contains(&top_neighbour) &&
            !segment.contains(&right_neighbour) &&
            segment.contains(&top_right_neighbour) {
            corner_count += 1;
        }

        if !segment.contains(&left_neighbour) &&
            !segment.contains(&bottom_neighbour) &&
            segment.contains(&bottom_left_neighbour) {
            corner_count += 1;
        }

        if !segment.contains(&right_neighbour) &&
            !segment.contains(&bottom_neighbour) &&
            segment.contains(&bottom_right_neighbour) {
            corner_count += 1;
        }

        if !segment.contains(&top_neighbour) &&
            !segment.contains(&left_neighbour) &&
            segment.contains(&top_left_neighbour) {
            corner_count += 1;
        }
    }

    corner_count
}

fn perimeter(segment: &Vec<(usize, usize)>) -> usize {
    let mut count = 0;
    for coord in segment {
        for direction in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_coord = (coord.0 as isize + direction.0, coord.1 as isize + direction.1);
            if new_coord.0 < 0 || new_coord.1 < 0 {
                count += 1;
                continue;
            }

            if !segment.contains(&(new_coord.0 as usize, new_coord.1 as usize)) {
                count += 1;
            }
        }
    }
    count
}