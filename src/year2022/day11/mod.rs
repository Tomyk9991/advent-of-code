use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Operation {
    operation: String
}

impl Operation {
    pub fn execute(&self, old: u128) -> u128 {
        return match self.operation.split("new = ").collect::<Vec<&str>>()[..][1].split(' ').collect::<Vec<&str>>()[..] {
            ["old", operation, "old"] => {
                let operation = operation.trim();

                match operation {
                    "*" => old * old,
                    "+" => old + old,
                    _ => panic!("Unexpected operation")
                }
            },
            ["old", operation, value] => {
                let operation = operation.trim();
                let value = value.trim().parse::<u128>().unwrap();
                match operation {
                    "*" => old * value,
                    "+" => old + value,
                    _ => panic!("Unexpected operation")
                }
            },
            [value, operation, "old"] => {
                let operation = operation.trim();
                let value = value.trim().parse::<u128>().unwrap();
                match operation {
                    "*" => value * old,
                    "+" => value + old,
                    _ => panic!("Unexpected operation")
                }
            },
            _ => panic!("Unexpected operation")
        };
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    starting_items: Vec<u128>,
    operation: Operation,
    test_divisible_by: u128,
    test_divisible_result: [usize; 2],
    inspections: u128
}

impl Monkey {
    pub fn increase_inspection(&mut self, amount: usize) {
        self.inspections += amount as u128;
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(target: &str) -> Result<Self, Self::Err> {
        let mut starting_items = vec![];
        let mut operation = Operation { operation: String::from("") };
        let mut test_divisible_by: u128 = 0;
        let mut test_divisible_result: [usize; 2] = [0, 0];

        let lines = target.lines().collect::<Vec<&str>>();
        for (i, line) in lines.iter().enumerate() {
            let value: &str = line.split(':').collect::<Vec<&str>>()[1].trim();
            if i == 1 { // starting items
                let matching = &value.split(',').collect::<Vec<&str>>()[..];
                match matching {
                    [worry_level] => {
                        starting_items.push((*worry_level).trim().parse::<u128>().unwrap());
                    },
                    _ => {
                        for worry_level in matching {
                            starting_items.push((*worry_level).trim().parse::<u128>().unwrap());
                        }
                    }
                }
            } else if i == 2 { // operation
                operation.operation = value.trim().to_string();
            } else if i == 3 {
                match value.split(' ').collect::<Vec<&str>>()[..] {
                    ["divisible", "by", number] => {
                        test_divisible_by = number.trim().parse::<u128>().unwrap();
                    },
                    _ => panic!("Divisible by")
                }
            } else if i == 4 {
                match value.split(' ').collect::<Vec<&str>>()[..] {
                    ["throw", "to", "monkey", monkey_index] => {
                        test_divisible_result[0] = monkey_index.trim().parse::<usize>().unwrap();
                    },
                    _ => panic!("test divisible result 0")
                }
            } else if i == 5 {
                match value.split(' ').collect::<Vec<&str>>()[..] {
                    ["throw", "to", "monkey", monkey_index] => {
                        test_divisible_result[1] = monkey_index.trim().parse::<usize>().unwrap();
                    },
                    _ => panic!("test divisible result 1")
                }
            }
        }

        Ok(Monkey {
            starting_items,
            operation,
            test_divisible_by,
            test_divisible_result,
            inspections: 0
        })
    }
}

pub struct Day11;
impl crate::year2022::Day for Day11 {
    fn date(&self) -> (i32, i32) { (11, 2022) }

    fn run(&self) {
        let source_monkeys: Vec<Monkey> = fs::read_to_string("src/year_2022/day11/input.txt").unwrap()
            .split("\r\n\r\n")
            .map(Monkey::from_str)
            .collect::<Result<_, _>>().expect("parsing");

        let mut monkeys: Vec<Monkey> = source_monkeys.clone();

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let monkey = monkeys[i].clone();

                for starting_item in &monkey.starting_items {
                    let worry_level = (monkey.operation.execute(*starting_item) as f64 / 3.0) as u128;
                    let new_monkey_idx = if worry_level % monkey.test_divisible_by == 0 {
                        monkey.test_divisible_result[0]
                    } else {
                        monkey.test_divisible_result[1]
                    };

                    monkeys[i].increase_inspection(1);
                    monkeys[new_monkey_idx].starting_items.push(worry_level);
                }

                monkeys[i].starting_items.clear();
            }
        }

        monkeys.sort_by(|a, b| {
            b.inspections.cmp(&a.inspections)
        });

        println!("Part one: {:?}", monkeys[0].inspections * monkeys[1].inspections);

        let mut monkeys: Vec<Monkey> = source_monkeys.clone();

        let common_multiple = monkeys.iter().map(|m| m.test_divisible_by).fold(1, |mut cm, x| {
            cm *= x;
            cm
        });

        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                let monkey = monkeys[i].clone();

                for starting_item in &monkey.starting_items {
                    let worry_level = monkey.operation.execute(*starting_item) % common_multiple;
                    let new_monkey_idx = if worry_level % monkey.test_divisible_by == 0 {
                        monkey.test_divisible_result[0]
                    } else {
                        monkey.test_divisible_result[1]
                    };

                    monkeys[i].increase_inspection(1);
                    monkeys[new_monkey_idx].starting_items.push(worry_level);
                }

                monkeys[i].starting_items.clear();
            }
        }
        monkeys.sort_by(|a, b| {
            b.inspections.cmp(&a.inspections)
        });

        println!("Part two: {}", monkeys[0].inspections * monkeys[1].inspections);
    }
}