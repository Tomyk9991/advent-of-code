use std::ops::Index;

#[derive(Default, Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y * self.width + x]
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

pub struct AsteriskIterator<'a> {
    pub grid: &'a Grid<char>,
    pub current_position: Coord,
    pub width: usize,
    pub height: usize,
}

impl Iterator for AsteriskIterator<'_> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        for y in self.current_position.1..self.height {
            for x in self.current_position.0..self.width {
                let current_char = self.grid[(x, y)];
                self.current_position = (x + 1, y);

                if current_char == '*' {
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