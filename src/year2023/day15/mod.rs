use std::fmt::Debug;
use std::str::FromStr;

use crate::aoc::Error;

#[derive(Debug, Default, Clone)]
enum Operation {
    #[default]
    Minus,
    Assign(usize),
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split(' ').collect::<Vec<&str>>()[..] {
            [_label, "=", number] => {
                Ok(Operation::Assign(number.parse::<usize>()?))
            }
            k if k.iter().any(|r| r.contains('-')) => {
                Ok(Operation::Minus)
            },
            _ => {
                Err(Error::StringParse("Could not parse".to_string()))
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
struct HashOperation {
    label: String,
    operation: Operation,
}

trait CustomHashOperation {
    fn custom_hash(&self) -> usize;
}

impl CustomHashOperation for String {
    fn custom_hash(&self) -> usize {
        let mut value: u32 = 0;

        for char in self.chars() {
            value += char as u32;
            value *= 17;
            value %= 256;
        }

        value as usize
    }
}

#[derive(Debug, Clone, Default)]
pub struct Day {
    sequences: Vec<String>,
    hash_operations: Vec<HashOperation>,
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("HASH", 52),
            ("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 1320),
        ]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 145)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut sum = 0;

        for value in &self.sequences {
            sum += value.custom_hash();
        }

        Ok(sum)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut boxes: Vec<Vec<Box>> = vec![vec![]; 256];

        for hash_operation in &self.hash_operations {
            let box_index = hash_operation.label.custom_hash();
            match hash_operation.operation {
                Operation::Minus => {
                    if let Some(index) = boxes[box_index].iter().position(|content| content.label == hash_operation.label) {
                        boxes[box_index].remove(index);
                    }
                }
                Operation::Assign(focal_length) => {
                    if let Some(index) = boxes[box_index].iter().position(|content| content.label == hash_operation.label) {
                        boxes[box_index][index].focal_length = focal_length;
                    } else {
                        boxes[box_index].push(Box {
                            label: &hash_operation.label,
                            focal_length,
                        })
                    }
                }
            }
        }


        let mut sum = 0;
        for (box_index, contents) in boxes.iter().enumerate() {
            for (content_index, content) in contents.iter().enumerate() {
                sum += (box_index + 1) * (content_index + 1) * content.focal_length;
            }
        }

        Ok(sum)
    }
}

#[derive(Default, Clone)]
struct Box<'a> {
    label: &'a str,
    focal_length: usize
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sequences: s.split(',').map(|a| a.to_string()).collect::<Vec<_>>(),
            hash_operations: s.split(',').map(|sequence| {
                HashOperation {
                    label: sequence
                        .split(&['=', '-'][..])
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()[0].clone(),
                    operation: Operation::from_str(&sequence
                        .replace('=', " = ")
                    ).unwrap_or_default(),
                }
            }).collect::<Vec<_>>(),
        })
    }
}