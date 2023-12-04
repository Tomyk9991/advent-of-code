use std::collections::HashSet;
use std::str::FromStr;

use crate::Error;
use crate::year2023::day03::grid::{AsteriskIterator, Coord, Grid, IteratorResult, NumberGridIterator};

mod grid;

#[derive(Default, Clone)]
pub struct Day {
    grid: Grid<char>,
}

impl crate::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..", 4361)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..", 467835)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let grid_iterator = NumberGridIterator {
            grid: &self.grid,
            width: self.grid.width,
            height: self.grid.height,
            current_position: (0, 0),
        };

        let mut sum = 0;
        for current_number_in_grid in grid_iterator {
            let mut found_special_character = false;
            'outer: for y in current_number_in_grid.position_range.0.1..=current_number_in_grid.position_range.1.1 {
                for x in current_number_in_grid.position_range.0.0..=current_number_in_grid.position_range.1.0 {

                    found_special_character = !get_neighbours_predicate(x, y, &self.grid, |c| !c.is_ascii_digit() && c != '.', false).is_empty();
                    if found_special_character {
                        break 'outer;
                    }
                }
            }

            if found_special_character {
                sum += current_number_in_grid.value;
            }
        }

        Ok(sum)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let asterisk_iterator = AsteriskIterator {
            grid: &self.grid,
            width: self.grid.width,
            height: self.grid.height,
            current_position: (0, 0),
        };


        let mut sum = 0;


        let all_numbers = NumberGridIterator {
            grid: &self.grid,
            width: self.grid.width,
            height: self.grid.height,
            current_position: (0, 0),
        }.collect::<Vec<_>>();

        for current_asterisk in asterisk_iterator {
            let x = current_asterisk.0;
            let y = current_asterisk.1;

            let values = get_neighbours_predicate(x, y, &self.grid, |c| c.is_ascii_digit(), true);

            if values.len() == 2 {
                let values = values.iter().collect::<Vec<_>>();
                let first_number = find_number_from_coord(&all_numbers, (values[0].0, values[0].1));
                let second_number = find_number_from_coord(&all_numbers, (values[1].0, values[1].1));

                if let (Some(first_number), Some(second_number)) = (first_number, second_number) {
                    sum += first_number.value * second_number.value;
                }
            }
        }

        Ok(sum)
    }
}

fn get_neighbours_predicate(x: usize, y: usize, grid: &Grid<char>, predicate: fn(char) -> bool, find_number: bool) -> Vec<Coord> {
    let mut values = HashSet::new();
    let mut coords_no_number_find = vec![];
    let all_numbers = NumberGridIterator {
        grid,
        width: grid.width,
        height: grid.height,
        current_position: (0, 0),
    }.collect::<Vec<_>>();

    // left
    if x != 0 && predicate(grid[(x - 1, y)]) {
        values.insert(find_number_from_coord(&all_numbers, (x - 1, y)));
        coords_no_number_find.push((x - 1, y))
    }
    // top left
    if x != 0 && y != 0 && predicate(grid[(x - 1, y - 1)]) {
        values.insert(find_number_from_coord(&all_numbers, (x - 1, y - 1)));
        coords_no_number_find.push((x - 1, y - 1))
    }
    // top
    if y != 0 && predicate(grid[(x, y - 1)]) {
        values.insert(find_number_from_coord(&all_numbers, (x, y - 1)));
        coords_no_number_find.push((x, y - 1))
    }
    // top right
    if x + 1 < grid.height && y != 0 && predicate(grid[(x + 1, y - 1)]) {
        values.insert(find_number_from_coord(&all_numbers, (x + 1, y - 1)));
        coords_no_number_find.push((x + 1, y - 1))
    }
    // right
    if x + 1 < grid.height && predicate(grid[(x + 1, y)]) {
        values.insert(find_number_from_coord(&all_numbers, (x + 1, y)));
        coords_no_number_find.push((x + 1, y))
    }
    // bottom right
    if x + 1 < grid.height && y + 1 < grid.width && predicate(grid[(x + 1, y + 1)]) {
        values.insert(find_number_from_coord(&all_numbers, (x + 1, y + 1)));
        coords_no_number_find.push((x + 1, y + 1))
    }
    // bottom
    if y + 1 < grid.width && predicate(grid[(x, y + 1)]) {
        values.insert(find_number_from_coord(&all_numbers, (x, y + 1)));
        coords_no_number_find.push((x, y + 1))
    }
    // bottom left
    if y + 1 < grid.width && x != 0 && predicate(grid[(x - 1, y + 1)]) {
        values.insert(find_number_from_coord(&all_numbers, (x - 1, y + 1)));
        coords_no_number_find.push((x - 1, y + 1));
    }

    if find_number {
        let mut vec = vec![];

        for value in values.into_iter().flatten() {
            vec.push(value.position_range.0)
        }

        vec
    } else {
        coords_no_number_find
    }
}

fn find_number_from_coord(vector: &Vec<IteratorResult>, number_to_find: (usize, usize)) -> Option<IteratorResult> {
    for iterator_result in vector {
        let x_range = iterator_result.position_range.0.0..=iterator_result.position_range.1.0;
        let y_range = iterator_result.position_range.0.1..=iterator_result.position_range.1.1;

        if x_range.contains(&number_to_find.0) && y_range.contains(&number_to_find.1) {
            return Some(iterator_result.clone());
        }
    }

    None
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::new(
            s.lines().collect::<Vec<_>>()[0].len(),
            s.lines().count(),
            s.lines().flat_map(|line| line.chars()).collect(),
        );
        Ok(Self {
            grid
        })
    }
}