use std::fs;
use std::ops::Index;

struct Grid {
    data: Vec<Vec<u32>>
}

impl Grid {
    pub fn from(input: &Vec<&str>) -> Self {
        let mut data: Vec<Vec<u32>> = vec![];

        for line in input {
            let mut row: Vec<u32> = vec![];
            for char in line.chars() {
                let digit = char.to_digit(10).unwrap();
                row.push(digit);
            }
            data.push(row);
        }

        Grid {
            data
        }
    }

    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        let current_value = self[(x, y)];
        // border
        if x == 0 || x == self.width() - 1 || y == 0 || y == self.height() - 1 {
            return true;
        }


        let mut blocked: u8 = 0b00000000;
        let x_ranges = [
            0..x,
            x + 1..self.width()
        ];

        let y_ranges = [
            0..y,
            y + 1..self.height()
        ];

        // blocked from horizontal
        for (i, x_range) in x_ranges.iter().enumerate() {
            for neighbour in x_range.start..x_range.end {
                if self[(neighbour, y)] >= current_value {
                    blocked |= 1 << i;
                }
            }
        }

        for (i, y_range) in y_ranges.iter().enumerate() {
            for neighbour in y_range.start..y_range.end {
                if self[(x, neighbour)] >= current_value {
                    blocked |= 1 << (i + 2);
                }
            }
        }

        blocked != 15
    }

    pub fn scenic_score(&self, x: usize, y: usize) -> usize {
        let current_value = self[(x, y)];

        let mut look_left = 0;
        for neighbour in (0..x).rev()  {
            if self[(neighbour, y)] >= current_value {
                look_left += 1;
                break;
            }

            look_left += 1;
        }

        let mut look_right = 0;
        for neighbour in x + 1..self.width()  {
            if self[(neighbour, y)] >= current_value {
                look_right += 1;
                break;
            }

            look_right += 1;
        }

        let mut loop_up = 0;
        for neighbour in (0..y).rev() {
            if self[(x, neighbour)] >= current_value {
                loop_up += 1;
                break;
            }

            loop_up += 1;
        }

        let mut look_down = 0;
        for neighbour in y + 1..self.height() {
            if self[(x, neighbour)] >= current_value {
                look_down += 1;
                break;
            }

            look_down += 1;
        }

        loop_up * look_down * look_left * look_right
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1][index.0]
    }
}

pub struct Day8;
impl crate::year2022::Day for Day8 {
    fn date(&self) -> (i32, i32) { (8, 2022) }

    fn run(&self) {
        let input = fs::read_to_string("src/year_2022/day8/input.txt").unwrap();

        let lines = input.split('\n').map(|line| {
            line.trim()
        }).collect::<Vec<&str>>();

        let grid: Grid = Grid::from(&lines);

        let mut counter = 0;
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                if grid.is_visible(x, y) {
                    counter += 1;
                }
            }
        }

        println!("Part one: {}", counter);

        let mut highest_scenic_score = 0;

        for x in 1..grid.width() - 1{
            for y in 1..grid.height() - 1 {
                let current_score = grid.scenic_score(x, y);
                if current_score > highest_scenic_score {
                    highest_scenic_score = current_score;
                }
            }
        }

        println!("Part two: {}", highest_scenic_score);
    }
}