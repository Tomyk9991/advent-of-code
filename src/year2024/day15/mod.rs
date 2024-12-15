use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use crate::utils::grid::Grid;


#[derive(Debug, Clone, Default)]
enum Instruction {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Default, Clone)]
pub struct Day {
    grid: Grid<char>,
    instructions: Vec<Instruction>
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsing_instructions = false;
        let mut grid_str = Vec::new();
        let mut instructions = Vec::new();

        for line in s.lines() {
            if line.is_empty() {
                parsing_instructions = true;
                continue;
            }

            if !parsing_instructions {
                grid_str.push(line.trim());
                continue;
            }

            for c in line.chars() {
                let instruction = match c {
                    '^' => Instruction::Top,
                    'v' => Instruction::Bottom,
                    '<' => Instruction::Left,
                    '>' => Instruction::Right,
                    _ => continue
                };
                instructions.push(instruction);
            }
        }

        let mut width = 0;
        let grid = grid_str.iter().map(|line| {
            width = line.len();
            line.chars().map(|c| c.into()).collect::<Vec<char>>()
        })
        .flatten()
        .collect::<Vec<char>>();
        let height = grid_str.len();

        Ok(Day {
            grid: Grid::new(width, height, grid),
            instructions
        })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#, 10092), (r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#, 2028)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#, 9021), (r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#, 618), /*(r#"#####
#.@.#
#.O.#
#.O.#
#...#
#.O.#
#...#
#...#
#...#
#####

vv"#, 1)*/]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut grid = self.grid.clone();

        for instruction in &self.instructions {
            let mut player_position = grid.find(|a| *a == '@').ok_or(crate::aoc::Error::NoSolutionFound)?;

            let next_position = match instruction {
                Instruction::Top => (player_position.0, player_position.1 - 1),
                Instruction::Bottom => (player_position.0, player_position.1 + 1),
                Instruction::Left => (player_position.0 - 1, player_position.1),
                Instruction::Right => (player_position.0 + 1, player_position.1),
            };

            if let Some(next_file) = grid.get((next_position.0, next_position.1)) {
                match next_file {
                    'O' => {
                        let mut next_next_position = match instruction {
                            Instruction::Top => (next_position.0, next_position.1 - 1),
                            Instruction::Bottom => (next_position.0, next_position.1 + 1),
                            Instruction::Left => (next_position.0 - 1, next_position.1),
                            Instruction::Right => (next_position.0 + 1, next_position.1),
                        };

                        if let Some(next_next_tile) = grid.get((next_next_position.0, next_next_position.1)) {
                            if *next_next_tile == '.' {
                                grid[(next_next_position.0, next_next_position.1)] = 'O';
                                grid[(next_position.0, next_position.1)] = '@';
                                grid[(player_position.0, player_position.1)] = '.';
                            } else if *next_next_tile == 'O' {
                                // move all the boxes to the next position
                                let mut next_next_next_position = next_next_position;

                                while let Some('O') = grid.get((next_next_next_position.0, next_next_next_position.1)) {
                                    next_next_next_position = match instruction {
                                        Instruction::Top => (next_next_position.0, next_next_position.1 - 1),
                                        Instruction::Bottom => (next_next_position.0, next_next_position.1 + 1),
                                        Instruction::Left => (next_next_position.0 - 1, next_next_position.1),
                                        Instruction::Right => (next_next_position.0 + 1, next_next_position.1),
                                    };

                                    if let Some('#') = grid.get((next_next_next_position.0, next_next_next_position.1)) {
                                        break;
                                    }

                                    if let Some('.') = grid.get((next_next_next_position.0, next_next_next_position.1)) {
                                        grid[(next_next_next_position.0, next_next_next_position.1)] = 'O';
                                        grid[(next_position.0, next_position.1)] = '@';
                                        grid[(player_position.0, player_position.1)] = '.';
                                        break;
                                    }

                                    next_next_position = next_next_next_position;
                                }
                            }
                        }
                    },
                    '.' => {
                        grid[(next_position.0, next_position.1)] = '@';
                        grid[(player_position.0, player_position.1)] = '.';
                        player_position = next_position;
                    },
                    _ => {}
                }
            }
        }

        let s = grid.find_all(|a| *a == 'O').iter().map(|a| 100 * a.1 + a.0).sum::<usize>();

        Ok(s as u64)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut grid = transform(&self.grid);

        for instruction in &self.instructions {
            let mut player_position = grid.find(|a| *a == '@').ok_or(crate::aoc::Error::NoSolutionFound)?;


            let next_position = match instruction {
                Instruction::Top => (player_position.0, player_position.1 - 1),
                Instruction::Bottom => (player_position.0, player_position.1 + 1),
                Instruction::Left => (player_position.0 - 1, player_position.1),
                Instruction::Right => (player_position.0 + 1, player_position.1),
            };

            if let Some(next_tile) = grid.get((next_position.0, next_position.1)) {
                let mut keep_track = HashSet::new();
                match next_tile {
                    '[' | ']' => {
                        keep_track.insert(next_position);
                        if *next_tile == '[' {
                            keep_track.insert((next_position.0 + 1, next_position.1));
                        } else if *next_tile == ']' {
                            keep_track.insert((next_position.0 - 1, next_position.1));
                        }

                        // watch in the direction
                        // move all the boxes to the next position
                        let mut look_at = VecDeque::new();
                        let mut seen = HashSet::new();
                        for current_box_position in &keep_track {
                            look_at.push_back(current_box_position.clone());
                        }

                        while let Some(current_box_fragment) = look_at.pop_front() {
                            seen.insert(current_box_fragment);

                            if let Some('.') = grid.get(current_box_fragment) {
                                continue;
                            }
                            let mut next_next_position = match instruction {
                                Instruction::Top => (current_box_fragment.0, current_box_fragment.1 - 1),
                                Instruction::Bottom => (current_box_fragment.0, current_box_fragment.1 + 1),
                                Instruction::Left => (current_box_fragment.0 - 1, current_box_fragment.1),
                                Instruction::Right => (current_box_fragment.0 + 1, current_box_fragment.1),
                            };

                            if let Some(a) = grid.get((next_next_position.0, next_next_position.1)) {
                                if *a == '#' {
                                    keep_track.clear();
                                    break;
                                }

                                if *a == '.' && look_at.len() == 0 {
                                    break;
                                }
                            }

                            keep_track.insert(next_next_position);


                            let next_next_tile = grid.get((next_next_position.0, next_next_position.1)).ok_or(crate::aoc::Error::NoSolutionFound)?;

                            if *next_next_tile == '[' {
                                keep_track.insert((next_next_position.0 + 1, next_next_position.1));
                                if !look_at.contains(&(next_next_position.0 + 1, next_next_position.1)) && !seen.contains(&(next_next_position.0 + 1, next_next_position.1)) {
                                    look_at.push_back((next_next_position.0 + 1, next_next_position.1));
                                }
                            } else if *next_next_tile == ']' {
                                keep_track.insert((next_next_position.0 - 1, next_next_position.1));
                                if !look_at.contains(&(next_next_position.0 - 1, next_next_position.1)) && !seen.contains(&(next_next_position.0 - 1, next_next_position.1)) {
                                    look_at.push_back((next_next_position.0 - 1, next_next_position.1));
                                }
                            }
                        }
                    },
                    '.' => {
                        grid[(next_position.0, next_position.1)] = '@';
                        grid[(player_position.0, player_position.1)] = '.';
                        player_position = next_position;
                    },
                    _ => {}
                }
                // shift everything in the "keep_track" to the next position
                let mut new_grid = grid.clone();
                if keep_track.len() % 2 != 0 {
                    let mut pairs: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
                    for p in &keep_track {
                        if let Some('[') = grid.get(p.clone()) {
                            if let Some(other_fragment) = keep_track.iter().find(|a| a.0 == p.0 + 1 && a.1 == p.1) {
                                pairs.insert((p.clone(), other_fragment.clone()));
                            }
                        }
                        if let Some(']') = grid.get(p.clone()) {
                            if let Some(other_fragment) = keep_track.iter().find(|a| a.0 == p.0 - 1 && a.1 == p.1) {
                                pairs.insert((p.clone(), other_fragment.clone()));
                            }
                        }
                    }

                    keep_track.clear();
                    for p in pairs {
                        keep_track.insert(p.0);
                        keep_track.insert(p.1);
                    }
                }
                for p in &keep_track {
                    new_grid[*p] = '.';
                }
                for position_to_shift in &keep_track {
                    let tile = *grid.get((position_to_shift.0, position_to_shift.1)).ok_or(crate::aoc::Error::NoSolutionFound)?;

                    let new_position = match instruction {
                        Instruction::Top => (position_to_shift.0, position_to_shift.1 - 1),
                        Instruction::Bottom => (position_to_shift.0, position_to_shift.1 + 1),
                        Instruction::Left => (position_to_shift.0 - 1, position_to_shift.1),
                        Instruction::Right => (position_to_shift.0 + 1, position_to_shift.1),
                    };

                    new_grid[new_position] = tile;
                }

                if !keep_track.is_empty() {
                    new_grid[player_position] = '.';
                    new_grid[next_position] = '@';
                }

                if new_grid.data.windows(2).any(|a| (a[0] == '[' && a[1] == '[') || (a[0] == ']' && a[1] == ']')) {
                    continue;
                }
                grid = new_grid;
            }
        }

        let s = grid.find_all(|a| *a == '[').iter().map(|a| 100 * a.1 + a.0).sum::<usize>();

        Ok(s as u64)
    }
}

fn transform(grid: &Grid<char>) -> Grid<char> {
    // If the tile is #, the new map contains ## instead.
    // If the tile is O, the new map contains [] instead.
    // If the tile is ., the new map contains .. instead.
    // If the tile is @, the new map contains @. instead.
    let mut transformed_data = vec![];
    let mut last_index = grid.data.len();

    grid.data.iter().enumerate().for_each(|(index, c)| {
        match c {
            '#' => {
                transformed_data.push('#');
                transformed_data.push('#');
            },
            'O' => {
                transformed_data.push('[');
                transformed_data.push(']');
            },
            '.' => {
                transformed_data.push('.');
                transformed_data.push('.');
            },
            '@' => {
                transformed_data.push('@');
                transformed_data.push('.');
            },
            _ => {}
        }
    });

    Grid::new(grid.width * 2, grid.height, transformed_data)
}