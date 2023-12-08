use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug};
use std::rc::{Rc, Weak};
use std::str::FromStr;

use crate::aoc::Error;
use crate::utils::tree::Node;

#[derive(Default, Clone, Debug)]
pub struct Day {
    com: Rc<RefCell<Node<String>>>,
}

impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L", 42)
        ]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN", 4)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let counts = count_all_child_to_root_path_lengths(self.com.clone());

        Ok(counts)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let start = self.com.borrow().search_in_tree_predicate(|a| a == "YOU");
        let end = self.com.borrow().search_in_tree_predicate(|a| a == "SAN");

        if let (Some(start), Some(end)) = (start, end) {
            let path = Node::advanced_breadth_first_search(start, end);
            return Ok(path.len() - 3)
        }

        Ok(0)
    }
}

fn count_all_child_to_root_path_lengths<T>(root: Rc<RefCell<Node<T>>>) -> usize {
    let current = root;

    let mut sum = 0;

    let children = current.borrow().children.clone();
    let mut queue: VecDeque<Rc<RefCell<Node<T>>>> = VecDeque::new();

    for child in children {
        let direct_and_indirect_orbits = traverse_from_child_to_root(Rc::clone(&child));
        sum += direct_and_indirect_orbits;

        queue.push_back(Rc::clone(&child));
    }

    while let Some(next) = queue.pop_front() {
        sum += count_all_child_to_root_path_lengths(next);
    }

    sum
}
fn traverse_from_child_to_root<T>(child: Rc<RefCell<Node<T>>>) -> usize {
    let mut counter = 0;
    let mut current_node = Some(child);

    while let Some(node) = current_node {
        if let Some(weak_parent) = node.borrow().parent.clone() {
            current_node = Weak::upgrade(&weak_parent);
            counter += 1;
        } else {
            current_node = None;
        }
    }

    counter
}


fn create_node(map: &HashMap<&str, Vec<&str>>, value: &str, parent: Option<Weak<RefCell<Node<String>>>>) -> Rc<RefCell<Node<String>>> {
    return if let Some(pointers) = map.get(&value) {
        let children = vec![];

        let child_parent = Rc::new(RefCell::new(Node {
            value: value.to_string(),
            parent,
            children,
        }));

        for child_value in pointers {
            let child = create_node(map, child_value, Some(Weak::clone(&Rc::downgrade(&child_parent))));
            child_parent.borrow_mut().children.push(child);
        }

        child_parent
    } else { // end of tree
        Rc::new(RefCell::new(Node {
            value: value.to_string(),
            parent,
            children: vec![],
        }))
    };
}


impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let orbits_relations = s.lines()
            .map(|line| line.split(')').collect::<Vec<_>>())
            .map(|line| (line[0], line[1]))
            .collect::<Vec<_>>();

        // to hashmap, which has to following: key and value, where value is a vector of all pointers
        let mut hashmap: HashMap<&str, Vec<&str>> = HashMap::new();

        for relation in orbits_relations {
            if let Some(children) = hashmap.get_mut(relation.0) {
                children.push(relation.1);
            } else {
                hashmap.insert(relation.0, vec![relation.1]);
            }
        }

        let com = create_node(&hashmap, "COM", None);

        Ok(Self {
            com,
        })
    }
}