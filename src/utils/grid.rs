use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Default, Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
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

impl Debug for Grid<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            write!(f, "")
        } else {
            let s = self.data.chunks(self.width).map(|a| a.iter().collect::<String>()).collect::<Vec<_>>();
            write!(f, "{}", s.join("\n"))
        }
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
}

impl<T: Clone> Grid<T> {
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

    pub fn find(&self, predicate: fn(&T) -> bool) -> Option<(usize, usize)> {
        let index = self.data.iter()
            .enumerate()
            .find_map(|(index, value)| predicate(value).then_some(index));

        if let Some(index) = index {
            return Some((index % self.width, index / self.width));
        }

        None
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
}