use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use crate::aoc::Error;
use crate::utils::tree::Node;

#[derive(Debug, Clone, Default)]
enum Instruction {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Clone, Default)]
pub struct Day {
    tree: Rc<RefCell<Node<String>>>,
    raw_data: String,
    lr_instructions: Vec<Instruction>,
}


impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)", 2),
            ("LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)", 6),
        ]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)", 6)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut current = Rc::clone(&self.tree);
        let destination = current.borrow().search_in_tree_predicate(|a| a == "ZZZ").ok_or(Error::StringParse(String::from("Cant find destination in tree")))?;
        let mut index = 0;
        let mut counter = 0;

        while current.as_ptr() != destination.as_ptr() {
            let s = match &self.lr_instructions[index] {
                Instruction::Left => 0,
                Instruction::Right => 1
            };

            let child = Rc::clone(&current.borrow().children[s]);
            current = child;
            index = (index + 1) % self.lr_instructions.len();

            counter += 1;
        }

        Ok(counter)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let k = Day::from_str_modified(&self.raw_data);
        let mut all_currents = k.0;
        let instructions = k.1;

        let mut index = 0;

        let mut counters = vec![TreeHelper { counter: 0, is_finished: false }; all_currents.len()];

        loop {
            let instruction = match &instructions[index] {
                Instruction::Left => 0,
                Instruction::Right => 1
            };

            for (current_index, current) in all_currents.iter_mut().enumerate() {
                if counters[current_index].is_finished {
                    continue;
                }

                let child = Rc::clone(&current.borrow().children[instruction]);
                *current = child;

                counters[current_index].counter += 1;

                if current.borrow().value.ends_with('Z') {
                    counters[current_index].is_finished = true;
                }
            }

            index = (index + 1) % instructions.len();

            if counters.iter().all(|m| m.is_finished) {
                break;
            }
        }

        Ok(lcm_list(&counters.iter().map(|m| m.counter).collect::<Vec<_>>()))
    }
}

#[derive(Clone)]
struct TreeHelper {
    counter: usize,
    is_finished: bool
}


fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn lcm_list(numbers: &[usize]) -> usize {
    numbers.iter().fold(1, |a, &b| lcm(a, b))
}


impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        let lri = iter.next().unwrap_or("");
        let mut instructions = vec![];

        for char in lri.chars() {
            instructions.push(match char {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => Instruction::Left
            });
        }


        let mut hash_map = HashMap::new();
        for line in s.lines() {
            if line.is_empty() || !line.contains('=') {
                continue;
            }

            let node_names = line
                .split('=')
                .map(|a| a.trim())
                .collect::<Vec<_>>();

            let node_name = node_names[0].to_string();
            let temp = node_names[1].split(", ").map(|a| a.replace(['(', ')'], "")).collect::<Vec<_>>();

            let childs: [String; 2] = [temp[0].to_string(), temp[1].to_string()];

            hash_map.insert(node_name, childs);
        }

        let root = create_node(&hash_map, "AAA", None, &mut HashMap::new());

        Ok(Self {
            tree: root,
            raw_data: s.to_string(),
            lr_instructions: instructions,
        })
    }
}

impl Day {
    #[allow(clippy::type_complexity)]
    fn from_str_modified(s: &str) -> (Vec<Rc<RefCell<Node<String>>>>, Vec<Instruction>) {
        let mut iter = s.lines();
        let lri = iter.next().unwrap_or("");
        let mut instructions = vec![];

        for char in lri.chars() {
            instructions.push(match char {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => Instruction::Left
            });
        }


        let mut hash_map = HashMap::new();
        for line in s.lines() {
            if line.is_empty() || !line.contains('=') {
                continue;
            }

            let node_names = line
                .split('=')
                .map(|a| a.trim())
                .collect::<Vec<_>>();

            let node_name = node_names[0].to_string();
            let temp = node_names[1].split(", ").map(|a| a.replace(['(', ')'], "")).collect::<Vec<_>>();

            let childs: [String; 2] = [temp[0].to_string(), temp[1].to_string()];

            hash_map.insert(node_name, childs);
        }

        let mut c = vec![];

        for key in hash_map.keys() {
            if key.ends_with('A') {
                let root = create_node(&hash_map, key, None, &mut HashMap::new());
                c.push(root);
            }
        }

        (c, instructions)
    }
}

fn create_node(map: &HashMap<String, [String; 2]>, value: &str, parent: Option<Weak<RefCell<Node<String>>>>, currently_build: &mut HashMap<String, Rc<RefCell<Node<String>>>>) -> Rc<RefCell<Node<String>>> {
    return if let Some(pointers) = map.get(value) {
        let children = vec![];

        let childs_parent = Rc::new(RefCell::new(Node {
            value: value.to_string(),
            parent,
            children,
        }));

        if !currently_build.contains_key(value) {
            currently_build.insert(value.to_string(), Rc::clone(&childs_parent));
        }


        for child_value in pointers {
            if let Some(already_build_child) = currently_build.get(child_value) {
                childs_parent.borrow_mut().children.push(Rc::clone(already_build_child));
            } else {
                let child = create_node(map, child_value, Some(Weak::clone(&Rc::downgrade(&childs_parent))), currently_build);
                currently_build.insert(child_value.to_string(), Rc::clone(&child));
                childs_parent.borrow_mut().children.push(child);
            }
        }

        childs_parent
    } else { // end of tree
        Rc::new(RefCell::new(Node {
            value: value.to_string(),
            parent,
            children: vec![],
        }))
    };
}