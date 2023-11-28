use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
struct Rope {
    positions: Vec<Position>,
    tail_size: usize
}

fn sign_without_0(value: isize) -> isize {
    if value >= 0 {
        1
    } else {
        -1
    }
}

impl Rope {
    pub fn new(tail_size: usize) -> Self {
        Self {
            positions: vec![Position { x: 0, y: 0 }; tail_size],
            tail_size
        }
    }


    pub fn adjust_x_gt_1(&mut self, curr_tail: usize) {
        let cur_x = self.positions[curr_tail].x;
        let prev_x = self.positions[curr_tail - 1].x;

        self.positions[curr_tail].x = prev_x + sign_without_0(cur_x - prev_x);
    }

    pub fn adjust_y_gt_1(&mut self, curr_tail: usize) {
        let cur_y = self.positions[curr_tail].y;
        let prev_y = self.positions[curr_tail - 1].y;

        self.positions[curr_tail].y = prev_y + sign_without_0(cur_y - prev_y);
    }

    pub fn traverse_tails(&mut self) {
        for curr_tail in 1..self.tail_size {
            let delta_x = (self.positions[curr_tail].x - self.positions[curr_tail - 1].x).abs();
            let delta_y = (self.positions[curr_tail].y - self.positions[curr_tail - 1].y).abs();

            if delta_x > 1 {
                self.adjust_x_gt_1(curr_tail);

                if delta_y > 0 {
                    if delta_y > 1 {
                        self.adjust_y_gt_1(curr_tail);
                    } else {
                        self.positions[curr_tail].y = self.positions[curr_tail - 1].y;
                    }
                }
            } else if delta_y > 1 {
                self.adjust_y_gt_1(curr_tail);

                if delta_x > 0 {
                    if delta_x > 1 {
                        self.adjust_x_gt_1(curr_tail);
                    } else {
                        self.positions[curr_tail].x = self.positions[curr_tail - 1].x;
                    }
                }
            }
        }
    }

    pub fn run_movements(&mut self, movements: &Vec<Movement>) -> Vec<Position> {
        let mut tail_positions = vec![*self.positions.last().unwrap()];

        for movement in movements {
            for _ in 0..movement.steps {
                match movement.direction {
                    Direction::Left => self.positions[0].x -= 1,
                    Direction::Right => self.positions[0].x += 1,
                    Direction::Up => self.positions[0].y -= 1,
                    Direction::Down => self.positions[0].y += 1,
                }

                self.traverse_tails();
                tail_positions
                    .push(*self.positions.last().unwrap());
            }
        }
        tail_positions
    }
}

struct Movement {
    direction: Direction,
    steps: usize,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}


pub struct Day9;


impl crate::year2022::Day for Day9 {
    fn date(&self) -> (i32, i32) { (9, 2022) }

    fn run(&self) {
        let moves = fs::read_to_string("src/year_2022/day9/input.txt")
            .unwrap()
            .lines()
            .map(|line| {
                if let [dir, unit] = line.split(' ').collect::<Vec<&str>>()[..] {
                    let direction = match dir {
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        "U" => Direction::Up,
                        "D" => Direction::Down,
                        _ => panic!("Direction not found"),
                    };

                    return Movement {
                        direction,
                        steps: unit.trim().parse::<usize>().unwrap(),
                    };
                }

                panic!("cant parse");
            }).collect::<Vec<Movement>>();

        let mut state = Rope::new(2);
        let positions = state.run_movements(&moves);
        let unique_positions: HashSet<Position> = positions.into_iter().collect();
        println!("Part one: {}", unique_positions.len());

        let mut state = Rope::new(10);
        let positions = state.run_movements(&moves);
        let unique_positions: HashSet<Position> = positions.into_iter().collect();
        println!("Part two: {}", unique_positions.len());
    }
}