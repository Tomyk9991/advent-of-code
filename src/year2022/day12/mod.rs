use std::collections::{HashMap, VecDeque};
use std::fs;
use std::str::FromStr;

struct Grid {
    starting_position: Pos,
    ending_position: Pos,
    width: usize,
    map: Vec<u8>,
}

impl Grid {
    fn neighbours(&self, pos: Pos) -> Vec<Direction> {
        let mut directions = Vec::new();
        let value = self.idx2d(&pos) + 1;

        if pos.x > 0 && self.idx2d(&Pos { x: pos.x - 1, y: pos.y }) <= value {
            directions.push(Direction::Left);
        }

        if pos.x < self.width - 1 && self.idx2d(&Pos { x: pos.x + 1, y: pos.y }) <= value {
            directions.push(Direction::Right);
        }

        if pos.y > 0 && self.idx2d(&Pos { x: pos.x, y: pos.y - 1 }) <= value {
            directions.push(Direction::Top);
        }
        if pos.y < self.map.len() / self.width - 1 && self.idx2d(&Pos { x: pos.x, y: pos.y + 1 }) <= value {
            directions.push(Direction::Bottom);
        }

        directions
    }

    fn idx2d(&self, pos: &Pos) -> u8 { self.map[pos.y * self.width + pos.x] }

    fn bfs(&self, start: Pos) -> Option<usize> {
        let mut visited = HashMap::new();
        visited.insert(start, 0);


        let mut q = VecDeque::new();
        q.push_back(start);

        while let Some(current) = q.pop_front() {
            for direction in self.neighbours(current) {
                let position = current.move_towards_direction(direction);

                #[allow(clippy::map_entry)]
                if !visited.contains_key(&position) {
                    visited.insert(position, visited[&current] + 1);
                    q.push_back(position);
                }
            }
        }

        visited.get(&self.ending_position).copied()
    }
}

enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos { x: usize, y: usize }

impl Pos {
    fn set(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

impl Pos {
    pub fn move_towards_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::Left => Self { x: self.x - 1, y: self.y },
            Direction::Right => Self { x: self.x + 1, y: self.y },
            Direction::Top => Self { x: self.x, y: self.y - 1 },
            Direction::Bottom => Self { x: self.x, y: self.y + 1 },
        }
    }
}


impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let mut map = Vec::new();
        let mut start = Pos { x: 0, y: 0};
        let mut end = Pos { x: 0, y: 0};


        s.lines().enumerate().for_each(|(y, line)| {
            if width.is_none() {
                width = Some(line.chars().count());
            }
            line.chars().enumerate().for_each(|(x, char)| {
                let height = match char {
                    'S' => {
                        start.set(x, y);
                        0
                    },
                    'E' => {
                        end.set(x, y);
                        b'z' - b'a'
                    }
                    _ => {
                        char as u8 - b'a'
                    }
                };

                map.push(height);

            });
        });

        Ok(Grid {
            starting_position: start,
            ending_position: end,
            width: width.unwrap(),
            map,
        })
    }
}


pub struct Day12;

impl crate::year2022::Day for Day12 {
    fn date(&self) -> (i32, i32) { (12, 2022) }

    fn run(&self) {
        // let mut grid: Grid = fs::read_to_string("src/year_2022/day12/test.txt")
        let grid: Grid = fs::read_to_string("src/year_2022/day12/input.txt")
            .unwrap()
            .parse()
            .unwrap();

        let shortest_path = grid.bfs(grid.starting_position).unwrap();
        println!("Part one: {shortest_path}");

        let mut minimum = usize::MAX;
        for y in 0..grid.map.len() / grid.width {
            for x in 0..grid.width {
                let pos = Pos { x, y };
                if grid.idx2d(&pos) == 0 {

                    if let Some(length) = grid.bfs(pos) {
                        if length < minimum {
                            minimum = length;
                        }
                    }
                }
            }
        }
        println!("Part two: {minimum}");
    }
}