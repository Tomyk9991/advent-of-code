use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Disk {
    Free(usize),
    Used(usize, usize) // Used(id, size)
}

#[derive(Default, Clone)]
pub struct Day {
    disk_map: Vec<Disk>
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec = vec![];

        for (index, char) in s.chars().enumerate() {
            vec.push(match index {
                current_index  if current_index % 2 == 0 => Disk::Used(index / 2, char.to_digit(10).ok_or(crate::aoc::Error::StringParse(char.to_string()))? as usize),
                _ => Disk::Free(char.to_digit(10).ok_or(crate::aoc::Error::StringParse(char.to_string()))? as usize),
            })
        }

        Ok(Self { disk_map: vec })
    }
}

#[derive(Debug)]
struct Block {
    free: usize,
    used: VecDeque<usize>
}

impl Block {
    fn has_free_space(&self) -> bool {
        self.free > 0
    }

    fn has_used(&self) -> bool {
        !self.used.is_empty()
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("2333133121414131402", 1928)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("2333133121414131402", 2858)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut target = vec![];

        for disk in self.disk_map.iter() {
            match disk {
                Disk::Used(id, size) => target.push(Block { free: 0, used: VecDeque::from(vec![*id; *size]) }),
                Disk::Free(size) => target.push(Block { free: *size, used: VecDeque::new() }),
            }
        }

        while let (Some(index_left), Some(index_right)) = (target.iter().position(|a| a.has_free_space()), target.iter().rposition(|a| a.has_used())) {
            if index_left >= index_right {
                break;
            }

            let left_value = {
                let right_block = &mut target[index_right];
                let value = right_block.used.pop_back();
                let cloned = value.clone();
                if matches!(cloned, Some(_)) {
                    right_block.free += 1;
                }

                value
            };

            if let Some(last_element) = left_value {
                let left_block = &mut target[index_left];
                left_block.free -= 1;
                left_block.used.push_back(last_element);
            }
        }

        let mut sum: u64 = 0;
        let mut counter = 0;
        for block in target.iter() {
            for block_id in block.used.iter() {
                sum += (counter as u64) * (*block_id as u64);
                counter += 1;
            }
        }

        Ok(sum)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut target = vec![];

        for disk in self.disk_map.iter() {
            match disk {
                Disk::Used(id, size) => target.push(Block { free: 0, used: VecDeque::from(vec![*id; *size]) }),
                Disk::Free(size) => target.push(Block { free: *size, used: VecDeque::new() }),
            }
        }

        let mut lowest_left: i32 = -1;
        let mut highest_right = target.len();

        let mut looked_up_ids = HashSet::new();

        while let (Some(index_left), Some(index_right)) = (
            target.iter().enumerate().position(|(index, a)| a.has_free_space() && index as i32 > lowest_left),
            target.iter().enumerate().rposition(|(index, a)| a.has_used() && index < highest_right)
        ) {
            lowest_left = index_left as i32;

            if index_left >= index_right {
                highest_right = index_right;
                lowest_left = -1;
                looked_up_ids.insert(target[index_right].used[0]);
            }

            if let Some(value) = target[index_right].used.get(0) {
                if looked_up_ids.contains(value) {
                    highest_right = index_right;
                    lowest_left = -1;
                    continue;
                }
            }

            if target[index_left].free >= target[index_right].used.len() {
                while target[index_left].free > 0 && target[index_right].used.len() > 0 {
                    let left_value = {
                        let right_block = &mut target[index_right];
                        let value = right_block.used.pop_back();
                        let cloned = value.clone();
                        if matches!(cloned, Some(_)) {
                            right_block.free += 1;
                        }

                        value
                    };

                    if let Some(last_element) = left_value {
                        let left_block = &mut target[index_left];
                        left_block.free -= 1;
                        looked_up_ids.insert(last_element);
                        left_block.used.push_back(last_element);
                    }
                }

                lowest_left = -1;
                highest_right = index_right;
            }

        }

        let mut sum: u64 = 0;
        let mut counter = 0;
        for block in target.iter() {
            for block_id in block.used.iter() {
                sum += (counter as u64) * (*block_id as u64);
                counter += 1;
            }
            counter += block.free as u64;
        }

        Ok(sum)
    }
}