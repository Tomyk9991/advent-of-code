use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::rc::{Rc, Weak};
use std::str::FromStr;
use crate::Error;

#[derive(Default, Clone, Debug)]
pub struct Day {
    com: Rc<RefCell<Node>>
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parent_name = if let Some(parent) = &self.parent {
            if let Some(p) = parent.upgrade() {
                p.borrow().value.clone()
            } else {
                "Weak Ref not found".to_string()
            }
        } else {
            "None".to_string()
        };

        f.debug_struct("Node")
            .field("value", &self.value)
            .field("parent", &parent_name)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Default, Clone)]
pub struct Node {
    value: String,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>
}

impl crate::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L", 42)
        ]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let counts = count_all_child_to_root_path_lengths(self.com.clone());

        Ok(counts)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        todo!()
    }
}

fn count_all_child_to_root_path_lengths(root: Rc<RefCell<Node>>) -> usize {
    let mut current = root;

    let mut sum = 0;

    let mut children = current.borrow().children.clone();

    let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();

    for child in children {
        let direct_and_indirect_orbits = traverse_from_child_to_root(Rc::clone(&child));
        sum += direct_and_indirect_orbits;

        queue.push_back(Rc::clone(&child));
    }

    while let Some(next) = queue.pop_front() {
        sum += count_all_child_to_root_path_lengths(next);
    }

    return sum;
}

fn traverse_from_child_to_root(child: Rc<RefCell<Node>>) -> usize {
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

fn create_node(map: &HashMap<&str, Vec<&str>>, name: &str, parent: Option<Weak<RefCell<Node>>>) -> Rc<RefCell<Node>> {
    return if let Some(pointers) = map.get(name) {
        let mut children = vec![];

        let mut child_parent = Rc::new(RefCell::new(Node {
            value: name.to_string(),
            parent,
            children,
        }));

        for child_name in pointers {
            let child = create_node(map, *child_name, Some(Weak::clone(&Rc::downgrade(&child_parent))));
            child_parent.borrow_mut().children.push(child);
        }

        child_parent
    } else { // end of tree
        Rc::new(RefCell::new(Node {
            value: name.to_string(),
            parent,
            children: vec![],
        }))
    }
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