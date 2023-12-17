use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::str::FromStr;

use crate::aoc::Error;
use crate::utils::grid::{Grid};

#[derive(Debug, Clone, Default)]
pub struct Day {
    grid: Grid<usize>
}


impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#, 102)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#, 94)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        if let Some(minimum_cost) = dijkstra_algorithm(&self.grid, 3, true, |movement, grid| {
            movement.x as usize == grid.width - 1 && movement.y as usize == grid.height - 1
        }) {
            return Ok(minimum_cost)
        }

        Err(Error::NoSolutionFound.into())
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        if let Some(minimum_cost) = dijkstra_algorithm(&self.grid, 10, false,|movement, grid| {
            movement.x as usize == grid.width - 1 && movement.y as usize == grid.height - 1 && movement.amount_steps >= 4
        }) {
            return Ok(minimum_cost)
        }

        Err(Error::NoSolutionFound.into())
    }
}

struct Movement {
    heat_loss: usize,
    y: isize,
    x: isize,
    direction_y: isize,
    direction_x: isize,
    amount_steps: usize
}

fn dijkstra_algorithm(grid: &Grid<usize>, min_steps: usize, push_allowed_without_constraint: bool, end_predicate: fn(&Movement, &Grid<usize>) -> bool) -> Option<usize> {
    let mut seen = HashSet::new();
    let mut priority_queue = BinaryHeap::new();

    priority_queue.push(Reverse(Movement {
        heat_loss: 0,
        y: 0,
        x: 0,
        direction_y: 0,
        direction_x: 0,
        amount_steps: 0,
    }));

    while let Some(Reverse(Movement { heat_loss, y, x, direction_y, direction_x, amount_steps})) = priority_queue.pop() {
        // dont include heat loss. if so and we end up in a loop, the heat_loss will grow, but the other parts are still the same
        if seen.contains(&(y, x, direction_y, direction_x, amount_steps)) {
            continue;
        }

        if end_predicate(&Movement {
            heat_loss,
            y,
            x,
            direction_y,
            direction_x,
            amount_steps,
        }, grid) {
            return Some(heat_loss);
        }

        seen.insert((y, x, direction_y, direction_x, amount_steps));

        if amount_steps < min_steps && (direction_y, direction_x) != (0, 0) {
            let next_y = y + direction_y;
            let next_x = x + direction_x;

            if grid.in_bounds(next_x, next_y) {
                priority_queue.push(Reverse(Movement {
                    heat_loss: heat_loss + grid[(next_x as usize, next_y as usize)],
                    y: next_y,
                    x: next_x,
                    direction_y,
                    direction_x,
                    amount_steps: amount_steps + 1,
                }))
            }
        }

        if push_allowed_without_constraint || amount_steps >= 4 || (direction_x, direction_y) == (0, 0) {
            for (next_direction_y, next_direction_x) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if (next_direction_y, next_direction_x) != (direction_y, direction_x) && (next_direction_y, next_direction_x) != (-direction_y, -direction_x){
                    let next_y = y + next_direction_y;
                    let next_x = x + next_direction_x;

                    if grid.in_bounds(next_x, next_y) {
                        priority_queue.push(Reverse(Movement {
                            heat_loss: heat_loss + grid[(next_x as usize, next_y as usize)],
                            y: next_y,
                            x: next_x,
                            direction_y: next_direction_y,
                            direction_x: next_direction_x,
                            amount_steps: 1
                        }))
                    }
                }
            }
        }
    }

    None
}


impl PartialOrd for Movement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heat_loss.partial_cmp(&other.heat_loss)
    }
}

impl Ord for Movement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Movement {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss == other.heat_loss
    }
}

impl Eq for Movement {}


impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.lines()
            .flat_map(|line| { line.chars() })
            .flat_map(|a| a.to_digit(10))
            .map(|a| a as usize)
            .collect::<Vec<_>>();

        let height = s.lines().count();
        let width = if let Some(line) = s.lines().next() {
            line.chars().count()
        } else {
            0
        };

        let grid: Grid<usize> = Grid::new(width, height, data);
        Ok(Self {  grid })
    }
}