use std::fmt::{Debug, Formatter};
use std::fs;

#[derive(Clone, PartialEq)]
enum Material {
    Rock,
    Air,
    Sand,
    SandSource,
}

struct Cave {
    rocks: Vec<Vec<Material>>,
    width: usize,
    height: usize,
    width_min: usize,
    unit: usize
}

enum MoveResult {
    Moved(Position),
    Settled,
    Void
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder: String = String::from("");

        for y in 0..self.height {
            for x in 0..self.width {
                builder += match self.rocks[y][x] {
                    Material::Rock => "#",
                    Material::Air => ".",
                    Material::Sand => "o",
                    Material::SandSource => "+",
                };
            }
            builder += "\n";
        }

        f.write_str(&builder)
    }
}

impl Cave {
    fn gtl(&self, global_pos: &Position) -> Position {
        Position {
            x: global_pos.x - self.width_min,
            y: global_pos.y,
        }
    }

    fn global_to_local(global_pos: &Position, width_min: usize) -> Position {
        Position {
            x: global_pos.x - width_min,
            y: global_pos.y,
        }
    }

    fn move_down(&self, position: &Position) -> MoveResult {
        if position.y + 1 >= self.height {
            return MoveResult::Void;
        }

        if self.rocks.get(position.y + 1).is_some() {
            if self.rocks[position.y + 1][position.x] == Material::Rock ||
                self.rocks[position.y + 1][position.x] == Material::Sand {
                return MoveResult::Settled;
            }


            MoveResult::Moved(Position { x: position.x, y: position.y + 1 })
        } else {
            MoveResult::Void
        }
    }

    fn move_down_left(&self, position: &Position) -> MoveResult {
        if position.x as i32 - 1 < 0 || position.y + 1 >= self.height {
            return MoveResult::Void;
        }

        if self.rocks.get(position.y + 1).is_some() && self.rocks[position.y + 1].get(position.x - 1).is_some() {
            if self.rocks[position.y + 1][position.x - 1] == Material::Rock ||
                self.rocks[position.y + 1][position.x - 1] == Material::Sand {
                return MoveResult::Settled;
            }

            MoveResult::Moved(Position { x: position.x - 1, y: position.y + 1 })
        } else {
            MoveResult::Void
        }
    }

    fn move_down_right(&self, position: &Position) -> MoveResult {
        if position.x + 1 >= self.width || position.y + 1 >= self.height {
            return MoveResult::Void;
        }

        if self.rocks.get(position.y + 1).is_some() && self.rocks[position.y + 1].get(position.x + 1).is_some() {
            if self.rocks[position.y + 1][position.x + 1] == Material::Rock ||
                self.rocks[position.y + 1][position.x + 1] == Material::Sand {
                return MoveResult::Settled;
            }

            MoveResult::Moved(Position { x: position.x + 1, y: position.y + 1 })
        } else {
            MoveResult::Void
        }
    }

    fn unit_move(&mut self) -> bool {
        let sand: Position = self.gtl(&Position {
            x: 500,
            y: 0
        });

        let mut voided = false;
        let mut current_pos = sand.clone();

        let mut move_sim = 0;

        loop {
            let mut count = 0;
            move_sim += 1;
            match self.move_down(&current_pos) {
                MoveResult::Moved(position) => { current_pos = position; continue; }
                MoveResult::Settled => { count += 1; }
                MoveResult::Void => { voided = true; break; }
            }

            match self.move_down_left(&current_pos) {
                MoveResult::Moved(position) => { current_pos = position; continue; }
                MoveResult::Settled => { count += 1; }
                MoveResult::Void => { voided = true; break; }
            }

            match self.move_down_right(&current_pos) {
                MoveResult::Moved(position) => { current_pos = position; }
                MoveResult::Settled => { count += 1; }
                MoveResult::Void => { voided = true; break; }
            }

            if count == 3 {
                if move_sim == 1 {
                    return false;
                }
                break;
            }
        }

        if voided {
            return false;
        }

        self.rocks[current_pos.y][current_pos.x] = Material::Sand;
        self.unit += 1;
        true
    }

    fn from_points(points: &Vec<Vec<Position>>) -> Self {
        let mut width_min: usize = usize::MAX;
        let height_min: usize = 0;

        let mut width: usize = usize::MIN;
        let mut height: usize = usize::MIN;


        for point_vec in points {
            for point in point_vec {
                // calc min bound
                if point.x < width_min {
                    width_min = point.x;
                }

                // calc max bound
                if point.x > width {
                    width = point.x;
                }

                if point.y > height {
                    height = point.y;
                }
            }
        }

        let mut rocks = vec![
            vec![Material::Air; width - width_min + 1]; height - height_min + 1
        ];


        let sand_source_local: Position = Cave::global_to_local(&Position{
            x: 500,
            y: 00
        }, width_min);


        rocks[sand_source_local.y][sand_source_local.x] = Material::SandSource;


        for point_vec in points {
            for i in 0..point_vec.len() - 1 {
                let point = &point_vec[i];

                let transformed: Position = Position {
                    x: point.x - width_min,
                    y: point.y,
                };

                let point = &point_vec[i + 1];
                let transformed_next: Position = Position {
                    x: point.x - width_min,
                    y: point.y,
                };

                let (lx, ux) = order(transformed.x, transformed_next.x);
                let (ly, uy) = order(transformed.y, transformed_next.y);

                for x in lx..=ux {
                    for item in rocks.iter_mut().take(uy + 1).skip(ly) {
                        item[x] = Material::Rock;
                    }
                }
            }
        }

        Cave {
            rocks,
            width: (width - width_min + 1),
            height: (height - height_min + 1),
            width_min,
            unit: 0
        }
    }
}

fn order(a: usize, b: usize) -> (usize, usize) {
    if a <= b { (a, b) } else { (b, a) }
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize
}

impl Position {
    fn from_str(s: &str) -> Vec<Self> {
        return s.split(" -> ").map(|coord| {
            if let [x, y] = coord.split(',').collect::<Vec<&str>>()[..] {
                return Position {x: x.parse::<usize>().unwrap(), y: y.parse::<usize>().unwrap() };
            }

            panic!("unexpected")
        }).collect::<Vec<Position>>();
    }
}

pub struct Day14;
impl crate::year2022::Day for Day14 {
    fn date(&self) -> (i32, i32) { (14, 2022) }

    fn run(&self) {
        let mut rocks = Vec::new();
        // let input= fs::read_to_string("src/year_2022/day14/test.txt")
        let input = fs::read_to_string("src/year_2022/day14/input.txt")
            .unwrap();

        for line in input.lines() {
            let rock_formation = Position::from_str(line);
            rocks.push(rock_formation);
        }

        let mut cave: Cave = Cave::from_points(&rocks);

        let mut units = 0;
        while cave.unit_move() {
            units += 1;
        }

        println!("Part 1: {}", units);

        let cave: Cave = Cave::from_points(&rocks);
        let inf_floor = cave.height + 1;

        let rock_formation = Position::from_str(&format!("0,{inf_floor} -> 1000,{inf_floor}"));
        rocks.push(rock_formation);

        let mut cave: Cave = Cave::from_points(&rocks);

        let mut units = 0;
        while cave.unit_move() {
            units += 1;
        }

        units += 1;

        println!("Part 2: {}", units);
    }
}