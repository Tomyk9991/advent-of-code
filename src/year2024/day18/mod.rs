use std::str::FromStr;
use itertools::Itertools;
use crate::utils::a_star::{a_star, a_star_in_place_grid, Direction, State};
use crate::utils::grid::{Coord, Distance, Grid};

#[derive(Default, Clone, Debug)]
pub struct Day {
    byte_positions: Vec<Coord>,
    width: usize,
    height: usize,
    amount_bytes: usize,
}


impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            byte_positions: s.lines().map(|line| {
                let poses = line.split(",").collect_vec();
                (poses[0].parse().unwrap_or(0), poses[1].parse().unwrap_or(0))
            }).collect_vec(),
            width: 7,
            height: 7,
            amount_bytes: 12,
        })
    }
}

impl crate::aoc::Day for Day {
    type Output = String;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0", 22.to_string())]
    }

    fn after_test_1(&mut self) {
        self.amount_bytes = 1024;
        self.width = 71;
        self.height = 71;
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0", "6,1".to_string())]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut grid = Grid::new(self.width, self.height, vec!['.'; self.width * self.height]);

        self.byte_positions.iter().take(self.amount_bytes).for_each(|(x, y)| {
            grid[(*x, *y)] =  '#';
        });

        let start = State {
            position: (0, 0),
            direction: Direction::North,
        };

        let h = |state: &State, goal: (usize, usize)| {
            // 0
            (state.position.0 as i32 - goal.0 as i32).abs() + (state.position.1 as i32 - goal.1 as i32).abs()
        };

        let g = |current: &State, next: &State| {
            current.position.manhattan_distance(&next.position) as i32
        };

        let f = |_state: &State, g: i32, h: i32| g + h;

        let get_neighbours = |state: &State| {
            let mut neighbors = Vec::new();
            let (x, y) = state.position;

            if grid.in_bounds(x as isize, y as isize - 1) && grid[(x, y - 1)] != '#' {
                neighbors.push(State { position: (x, y - 1), direction: state.direction.clone() });
            }
            if grid.in_bounds(x as isize + 1, y as isize) && grid[(x + 1, y)] != '#' {
                neighbors.push(State { position: (x + 1, y), direction: state.direction.clone() });
            }
            if grid.in_bounds(x as isize, y as isize + 1) && grid[(x, y + 1)] != '#' {
                neighbors.push(State { position: (x, y + 1), direction: state.direction.clone() });
            }
            if grid.in_bounds(x as isize - 1, y as isize) && grid[(x - 1, y)] != '#' {
                neighbors.push(State { position: (x - 1, y), direction: state.direction.clone() });
            }

            neighbors
        };



        let end = ((self.width as i32 - 1) as usize, (self.height as i32 - 1) as usize);

        let result = a_star(start, end, h, g, f, get_neighbours);
        if let Some((_, g_cost)) = result {
            return Ok(g_cost.to_string());
        }

        Ok("".to_string())
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut grid = Grid::new(self.width, self.height, vec!['.'; self.width * self.height]);

        self.byte_positions.iter().take(self.amount_bytes).for_each(|(x, y)| {
            grid[(*x, *y)] =  '#';
        });

        let start = State {
            position: (0, 0),
            direction: Direction::North,
        };
        let end = ((self.width as i32 - 1) as usize, (self.height as i32 - 1) as usize);

        let h = |state: &State, goal: (usize, usize)| {
            // 0
            (state.position.0 as i32 - goal.0 as i32).abs() + (state.position.1 as i32 - goal.1 as i32).abs()
        };

        let g = |current: &State, next: &State| {
            current.position.manhattan_distance(&next.position) as i32
        };

        let f = |_state: &State, g: i32, h: i32| g + h;


        let mut iter = self.byte_positions.iter().skip(self.amount_bytes);
        while let Some((x, y)) = iter.next() {
            grid[(*x, *y)] = '#';

            let result = a_star_in_place_grid(start.clone(), end, h, g, f, &grid);
            if let None = result {
                return Ok(format!("{},{}", x, y));
            }
        }


        Ok("".to_string())
    }
}