use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn distance(&self, other: &Position) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

#[derive(Debug)]
struct SensorBeaconPair {
    sensor: Position,
    beacon: Position
}


impl SensorBeaconPair {
    fn distance(&self) -> u32 {
        self.sensor.distance(&self.beacon)
    }
}

impl FromStr for SensorBeaconPair {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let ["Sensor", "at", s_x_eq, s_y_eq, "closest", "beacon", "is", "at", b_x_eq, b_y_eq] = line.split(' ').collect::<Vec<&str>>()[..] {
            let x_sensor: i32 = s_x_eq.split('=').collect::<Vec<&str>>()[1].replace(',', "").trim().parse::<i32>().unwrap();
            let y_sensor: i32 = s_y_eq.split('=').collect::<Vec<&str>>()[1].replace(':', "").trim().parse::<i32>().unwrap();

            let x_beacon: i32 = b_x_eq.split('=').collect::<Vec<&str>>()[1].replace(',', "").trim().parse::<i32>().unwrap();
            let y_beacon: i32 = b_y_eq.split('=').collect::<Vec<&str>>()[1].replace(':', "").trim().parse::<i32>().unwrap();

            return Ok(SensorBeaconPair{
                sensor: Position { x: x_sensor, y: y_sensor },
                beacon: Position { x: x_beacon, y: y_beacon },
            });
        }

        Err(String::from("Failed"))
    }
}

fn count_free_in_row(sensors: &Vec<SensorBeaconPair>, row: i32) -> usize {
    let block_row_positions: HashSet<i32> =
    sensors.iter()
        .map(|p| &p.sensor)
        .filter(|s| s.y == row)
        .map(|n| n.x)
        .chain(sensors.iter().map(|n| &n.beacon).filter(|n| n.y == row).map(|n| n.x))
        .collect();


    let mut blocked_cells = HashSet::new();

    for pair in sensors {
        let r = pair.distance() as i32;
        let row_dst = (row - pair.sensor.y).abs();

        if row_dst <= r {
            let w = r - row_dst;
            for i in -w..w + 1 {
                if block_row_positions.contains(&(pair.sensor.x + i)) {
                    continue;
                }

                blocked_cells.insert(pair.sensor.x + i);
            }
        }
    }

    blocked_cells.len()
}

fn pattern_from_center(sensor: &Position, radius: i32, area: i32) -> Vec<Position> {
    static DIRS: [Position; 4] = [
        Position { x: 1, y: -1},
        Position { x: 1, y: 1},
        Position { x: -1, y: 1},
        Position { x: -1, y: -1}
    ];

    let mut pos = sensor.clone();
    pos.x -= radius;
    let mut result = Vec::new();
    for dir in &DIRS {
        for _ in 0..radius as usize {
            if pos.x < 0 || pos.y < 0 || pos.x > area || pos.y > area {
                continue;
            }

            result.push(pos.clone());
            pos.x += dir.x;
            pos.y += dir.y;
        }
    }

    result
}

fn find_signal(data: &Vec<SensorBeaconPair>, area: i32) -> Position {
    for pair in data {
        for p in pattern_from_center(&pair.sensor, (pair.distance() + 1) as i32, area) {
            if !data.iter().any(|n| n.sensor.distance(&p) <= n.sensor.distance(&n.beacon)) {
                return p;
            }
        }
    }

    panic!();
}

pub struct Day15;
impl crate::year2022::Day for Day15 {
    fn date(&self) -> (i32, i32) {
        (15, 2022)
    }

    fn run(&self) {
        // let input = fs::read_to_string("src/year_2022/day15/test.txt")
        let input = fs::read_to_string("src/year_2022/day15/input.txt")
            .unwrap()
            .lines()
            .map(SensorBeaconPair::from_str)
            .map(|r| r.unwrap())
            .collect::<Vec<SensorBeaconPair>>();

        let target_row = 2000000;

        let result = count_free_in_row(&input, target_row);
        println!("Part one: {}", result);

        let pos = find_signal(&input, 4000000);
        println!("Part two: {}", pos.x as u64 * 4000000_u64 + pos.y as u64);
    }
}