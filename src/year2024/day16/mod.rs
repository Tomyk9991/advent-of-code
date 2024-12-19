use std::collections::HashSet;
use std::str::FromStr;
use crate::utils::a_star_impl::{a_star, a_star_all_paths, Direction, StateDirection};
use crate::utils::grid::Grid;


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
                line.chars().map(|c| c.into()).collect::<Vec<char>>()
            })
            .flatten()
            .collect::<Vec<char>>();

        let height = s.lines().count();
        Ok(Self { grid: Grid::new(width, height, grid) })
    }
}

impl crate::aoc::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("####\n#S.#\n#.E#\n####", 1002), (r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#, 7036)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#, 45), (r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#, 64)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let start_position = self.grid.find(|a| *a == 'S').ok_or(crate::aoc::Error::NoSolutionFound)?;

        let start = StateDirection {
            position: start_position,
            direction: Direction::East,
        };

        let goal = self.grid.find(|a| *a == 'E').ok_or(crate::aoc::Error::NoSolutionFound)?;

        let h = |state: &StateDirection, goal: (usize, usize)| {
            // 0
            (state.position.0 as i32 - goal.0 as i32).abs() + (state.position.1 as i32 - goal.1 as i32).abs()
        };

        let g = |current: &StateDirection, next: &StateDirection| {
            if current.position == next.position { 1000 } else { 1 }
        };

        let f = |_state: &StateDirection, g: i32, h: i32| g + h;

        let get_neighbours = |state: &StateDirection| {
            let mut neighbors = Vec::new();
            let (x, y) = state.position;

            match state.direction {
                Direction::North if self.grid.in_bounds(x as isize, y as isize - 1) && self.grid[(x, y - 1)] != '#' => {
                    neighbors.push(StateDirection { position: (x, y - 1), direction: state.direction.clone() });
                }
                Direction::East if self.grid.in_bounds(x as isize + 1, y as isize) && self.grid[(x + 1, y)] != '#' => {
                    neighbors.push(StateDirection { position: (x + 1, y), direction: state.direction.clone() });
                }
                Direction::South if self.grid.in_bounds(x as isize, y as isize + 1) && self.grid[(x, y + 1)] != '#' => {
                    neighbors.push(StateDirection { position: (x, y + 1), direction: state.direction.clone() });
                }
                Direction::West if self.grid.in_bounds(x as isize - 1, y as isize) && self.grid[(x - 1, y)] != '#' => {
                    neighbors.push(StateDirection { position: (x - 1, y), direction: state.direction.clone() });
                }
                _ => {}
            }

            // Rotate clockwise
            neighbors.push(StateDirection {
                position: state.position,
                direction: match state.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                },
            });

            // Rotate counterclockwise
            neighbors.push(StateDirection {
                position: state.position,
                direction: match state.direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                },
            });

            neighbors
        };

        let (_, cost) = &a_star(start, goal, h, g, f, get_neighbours).ok_or(crate::aoc::Error::NoSolutionFound)?;
        Ok(*cost)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        // instead of using came_from, the state itself stores the previous state
        let start_position = self.grid.find(|a| *a == 'S').ok_or(crate::aoc::Error::NoSolutionFound)?;

        let start = StateDirection {
            position: start_position,
            direction: Direction::East,
        };

        let goal = self.grid.find(|a| *a == 'E').ok_or(crate::aoc::Error::NoSolutionFound)?;

        let h = |state: &StateDirection, goal: (usize, usize)| {
            (state.position.0 as i32 - goal.0 as i32).abs() + (state.position.1 as i32 - goal.1 as i32).abs()
        };

        let g = |current: &StateDirection, next: &StateDirection| {
            if current.position == next.position { 1000 } else { 1 }
        };

        let f = |_state: &StateDirection, g: i32, h: i32| g + h;

        let get_neighbours = |state: &StateDirection| {
            let mut neighbors = Vec::new();
            let (x, y) = state.position;

            match state.direction {
                Direction::North if self.grid.in_bounds(x as isize, y as isize - 1) && self.grid[(x, y - 1)] != '#' => {
                    neighbors.push(StateDirection { position: (x, y - 1), direction: state.direction.clone() });
                }
                Direction::East if self.grid.in_bounds(x as isize + 1, y as isize) && self.grid[(x + 1, y)] != '#' => {
                    neighbors.push(StateDirection { position: (x + 1, y), direction: state.direction.clone() });
                }
                Direction::South if self.grid.in_bounds(x as isize, y as isize + 1) && self.grid[(x, y + 1)] != '#' => {
                    neighbors.push(StateDirection { position: (x, y + 1), direction: state.direction.clone() });
                }
                Direction::West if self.grid.in_bounds(x as isize - 1, y as isize) && self.grid[(x - 1, y)] != '#' => {
                    neighbors.push(StateDirection { position: (x - 1, y), direction: state.direction.clone() });
                }
                _ => {}
            }

            // Rotate clockwise
            neighbors.push(StateDirection {
                position: state.position,
                direction: match state.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                },
            });

            // Rotate counterclockwise
            neighbors.push(StateDirection {
                position: state.position,
                direction: match state.direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                },
            });

            neighbors
        };

        let (paths, _) = &a_star_all_paths(start, goal, h, g, f, get_neighbours).ok_or(crate::aoc::Error::NoSolutionFound)?;
        let mut unique_positions = HashSet::new();
        for path in paths {
            for state in path {
                unique_positions.insert(state.position);
                self.grid[state.position] = '0';
            }
        }

        Ok(unique_positions.len() as i32)
    }
}