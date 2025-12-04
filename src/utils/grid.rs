use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Index, IndexMut};
use std::slice::Iter;
use thiserror::Error;

#[derive(Default, Clone, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn adjacent_coords_8(&self, x: usize, y: usize) -> Vec<Coord> {
        let mut coords = Vec::new();

        for dy in -1isize..=1 {
            for dx in -1isize..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let new_x = x as isize + dx;
                let new_y = y as isize + dy;

                if self.in_bounds(new_x, new_y) {
                    coords.push((new_x as usize, new_y as usize));
                }
            }
        }

        coords
    }
}

#[derive(Debug, Error)]
pub enum Error {
    OutOfBounds(Coord),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Out of bounds: {:?}", self)
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            write!(f, "")
        } else {
            let s = self.data.chunks(self.width).map(|a| format!("{:?}", a)).collect::<Vec<_>>();
            write!(f, "{}", s.join("\n"))
        }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            write!(f, "")
        } else {
            let s = self.data.chunks(self.width).map(|a| a.iter().map(|a| a.to_string()).collect::<Vec<_>>().join("").to_string()).collect::<Vec<_>>();
            write!(f, "{}", s.join("\n"))
        }
    }
}


impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

pub enum Dimension {
    X,
    Y
}

pub trait Distance {
    fn euclidean_distance(&self, other: &Self) -> f32;
    fn manhattan_distance(&self, other: &Self) -> usize;
}

impl Distance for Coord {
    fn euclidean_distance(&self, other: &Self) -> f32 {
        (((other.0 - self.0).pow(2) + (other.1 - self.1).pow(2)) as f32).sqrt()
    }

    fn manhattan_distance(&self, other: &Self) -> usize {
        (self.0.abs_diff(other.0)) + (self.1.abs_diff(other.1))
    }
}

impl<T> Grid<T> {
    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T: Clone> Grid<T> {
    pub fn transpose(&self) -> Grid<T> {
        let mut transposed_data = Vec::with_capacity(self.data.len());

        for col in 0..self.width {
            for row in 0..self.height {
                transposed_data.push(self.data[row * self.width + col].clone());
            }
        }

        Grid {
            width: self.height,
            height: self.width,
            data: transposed_data,
        }
    }

    pub fn to_2d(&self) -> Vec<Vec<T>> {
        let mut result = Vec::with_capacity(self.height);

        for y in 0..self.height {
            let start_index = y * self.width;
            let end_index = start_index + self.width;

            let row = self.data[start_index..end_index].to_vec();
            result.push(row);
        }

        result
    }

    /// floorfills the grid. it looks for identical values and stores the positions in a vector. the returnvalue is a Vec<Vec<Coord>> where the inner vector contains all positions of the connected values
    pub fn floor_fill(&self, is_adjacent: impl Fn(&T, &T) -> bool) -> Vec<Vec<(usize, usize)>> {
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut result = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if visited[y][x] {
                    continue;
                }

                let mut stack = vec![(x, y)];
                let mut current_group = Vec::new();
                let current_segment_value = self[(x, y)].clone();

                while let Some((x, y)) = stack.pop() {
                    if visited[y][x] {
                        continue;
                    }

                    visited[y][x] = true;
                    current_group.push((x, y));

                    for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let new_x = x as isize + dx;
                        let new_y = y as isize + dy;

                        if self.in_bounds(new_x, new_y) && !visited[new_y as usize][new_x as usize] && is_adjacent(&current_segment_value, &self[(new_x as usize, new_y as usize)]) {
                            stack.push((new_x as usize, new_y as usize));
                        }
                    }
                }

                result.push(current_group);
            }
        }

        result
    }

    pub fn find(&self, predicate: fn(&T) -> bool) -> Option<(usize, usize)> {
        let index = self.data.iter()
            .enumerate()
            .find_map(|(index, value)| predicate(value).then_some(index));

        if let Some(index) = index {
            return Some((index % self.width, index / self.width));
        }

        None
    }

    pub fn find_all(&self, predicate: impl Fn(&T) -> bool) -> Vec<(usize, usize)> {
        self.data.iter()
            .enumerate()
            .filter_map(|(index, value)| predicate(value).then_some((index % self.width, index / self.width)))
            .collect()
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        if self.in_bounds(coord.0 as isize, coord.1 as isize) {
            Some(&self.data[coord.1 * self.width + coord.0])
        } else {
            None
        }
    }
}


impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[y * self.width + x]
    }
}

pub type Coord = (usize, usize);

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Self {
        Vec2 {
            x,
            y
        }
    }
}
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


impl Add for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct IteratorResult {
    /// value it found
    pub value: usize,
    /// position range how long all numbers are
    /// for example: 444 has position 0..0 to including 0..2
    pub position_range: (Coord, Coord),
}

pub struct NumberGridIterator<'a> {
    pub grid: &'a Grid<char>,
    pub current_position: Coord,
    pub width: usize,
    pub height: usize,
}

pub struct CharIterator<'a> {
    pub grid: &'a Grid<char>,
    pub current_position: Coord,
    pub target: char,
}

impl Iterator for CharIterator<'_> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        for y in self.current_position.1..self.grid.height {
            for x in self.current_position.0..self.grid.width {
                let current_char = self.grid[(x, y)];
                self.current_position = (x + 1, y);

                if current_char == self.target {
                    return Some((x, y));
                }
            }

            self.current_position.0 = 0;
        }

        None
    }
}

impl Iterator for NumberGridIterator<'_> {
    type Item = IteratorResult;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_number = "".to_string();
        let mut start: Option<Coord> = None;
        let mut end: Option<Coord> = None;

        for y in self.current_position.1..self.height {
            for x in self.current_position.0..self.width {
                let current_char = self.grid[(x, y)];
                self.current_position = (x + 1, y);

                if current_char.is_ascii_digit() {
                    if start.is_none() {
                        start = Some((x, y));
                    }

                    end = Some((x, y));
                    current_number.push(current_char);
                } else if !current_number.is_empty() {
                    return Some(IteratorResult {
                        value: current_number.parse::<usize>().unwrap_or(0),
                        position_range: (start.unwrap_or((0, 0)), end.unwrap_or((0, 0))),
                    });
                }
            }

            self.current_position.0 = 0;
        }

        None
    }
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn from_raw_input(input: &str) -> Grid<char> {
        Grid::<char>::new(
            input.lines().collect::<Vec<_>>()[0].len(),
            input.lines().count(),
            input.lines().flat_map(|line| line.chars()).collect(),
        )
    }
}