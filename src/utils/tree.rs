use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::rc::{Rc, Weak};
use itertools::Itertools;
use crate::utils::grid::{Coord, Grid};

#[derive(Default, Clone)]
pub struct Node<T> {
    pub value: T,
    pub parent: Option<Weak<RefCell<Node<T>>>>,
    pub children: Vec<Rc<RefCell<Node<T>>>>,
}


impl<T: Debug + Clone> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parent_name = if let Some(parent) = &self.parent {
            if let Some(p) = parent.upgrade() {
                Some(p.borrow().value.clone())
            } else {
                None
            }
        } else {
            None
        };


        let mut k = f.debug_struct("Node");
        k.field("value", &self.value);
        k.field("parent", &parent_name);
        k.field("children", &self.children.iter().map(|c| c.borrow()).collect::<Vec<_>>());

        k.finish()
    }
}


impl<T: Clone + PartialEq + Debug> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, value: T) {
        self.children.push(Rc::new(RefCell::new(Node::new(value.clone()))));
    }

    pub fn try_from_grid(grid: &Grid<T>, predicate: impl Fn(&T) -> bool, is_adjacent: impl Fn(&T, &T, &Coord) -> bool, overlapping_trees: bool) -> Vec<Rc<RefCell<Node<T>>>> {
        let mut nodes = Vec::new();
        let starting_points = grid.find_all(&predicate);

        for (x, y) in starting_points {
            if let Some(start_value) = grid.get((x, y)) {
                let root = Rc::new(RefCell::new(Node {
                    value: start_value.clone(),
                    parent: None,
                    children: Vec::new(),
                }));

                let mut visited = vec![vec![false; grid.width()]; grid.height()];
                visited[y][x] = true;

                Self::build_graph(grid, &is_adjacent, root.clone(), &mut visited, x, y, overlapping_trees);

                nodes.push(root);
            }
        }

        nodes
    }

    fn build_graph(grid: &Grid<T>, is_adjacent: &impl Fn(&T, &T, &Coord) -> bool, current: Rc<RefCell<Node<T>>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize, overlapping_trees: bool) {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // Cardinal directions

        for &(dx, dy) in &directions {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if nx < grid.width() && ny < grid.height() && (overlapping_trees || !visited[ny][nx]) {
                if let Some(neighbor_value) = grid.get((nx, ny)) {
                    let current_value = current.borrow().value.clone();
                    let coord = (nx, ny);

                    if is_adjacent(&current_value, neighbor_value, &coord) {
                        let child = Rc::new(RefCell::new(Node {
                            value: neighbor_value.clone(),
                            parent: Some(Rc::downgrade(&current)),
                            children: Vec::new(),
                        }));

                        current.borrow_mut().children.push(Rc::clone(&child));

                        visited[ny][nx] = true;
                        Self::build_graph(grid, is_adjacent, child, visited, nx, ny, overlapping_trees);
                    }
                }
            }
        }
    }

    /// Breadth first search but it also traverses in parent direction
    pub fn advanced_breadth_first_search(start: Rc<RefCell<Node<T>>>, end: Rc<RefCell<Node<T>>>) -> Vec<Rc<RefCell<Node<T>>>> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();

        let start_parent = if let Some(start_parent) = start.borrow().clone().parent {
            start_parent.upgrade()
        } else {
            None
        };

        queue.push_back((start.clone(), start_parent));

        while let Some((current, parent)) = queue.pop_front() {
            if Rc::ptr_eq(&current, &end) {
                let mut path = vec![current.clone()];
                let mut previous: Option<Rc<RefCell<Node<T>>>> = parent;

                // backtrace from 'end' node to 'start'
                while let Some(prev_node) = previous {
                    path.push(prev_node.clone());
                    let s = visited.get(&(Rc::as_ptr(&prev_node)));
                    if let Some(s) = s {
                        previous = Some(Rc::clone(s));
                    } else {
                        previous = None;
                    }

                    if Rc::ptr_eq(&prev_node, &start) {
                        break;
                    }
                }

                path.reverse();
                return path;
            }

            if let Some(parent) = parent {
                visited.insert(Rc::as_ptr(&current), Rc::clone(&parent));
            }

            let current_borrow = current.borrow();

            if let Some(ref parent_weak) = current_borrow.parent {
                if let Some(parent) = parent_weak.upgrade() {
                    if !visited.contains_key(&Rc::as_ptr(&parent)) {
                        queue.push_back((parent.clone(), Some(Rc::clone(&current))));
                    }
                }
            }

            for child in &current_borrow.children {
                if !visited.contains_key(&Rc::as_ptr(child)) {
                    queue.push_back((child.clone(), Some(Rc::clone(&current))));
                }
            }
        }

        Vec::new() // Return empty vector if no path found
    }


    pub fn search_in_tree_predicate(&self, predicate: fn(&T) -> bool) -> Option<Rc<RefCell<Node<T>>>> {
        self.search_in_tree_accumulated(predicate, &mut HashSet::new())
    }

    fn search_in_tree_accumulated(&self, predicate: fn(&T) -> bool, hashset: &mut HashSet<*mut Node<T>>) -> Option<Rc<RefCell<Node<T>>>> {
        let children = self.children.clone();

        for child in children {
            if hashset.contains(&child.as_ptr()) {
                continue;
            }

            hashset.insert(child.as_ptr());

            if predicate(&child.borrow().value) {
                return Some(child);
            }

            if let Some(found) = child.borrow().search_in_tree_accumulated(predicate, hashset) {
                return Some(found);
            }
        }

        None
    }
}

pub fn search_in_tree_predicate_all<T: Display + Hash + Debug + Eq + PartialEq + Clone>(own: &Rc<RefCell<Node<T>>>, predicate: fn(&T) -> bool) -> Vec<Rc<RefCell<Node<T>>>> {
    let mut a = vec![];

    search_in_tree_accumulated_all(own, predicate, &mut HashSet::new(), &mut a);
    a
}

fn search_in_tree_accumulated_all<T: Display + Hash + Debug + Eq + PartialEq + Clone>(own: &Rc<RefCell<Node<T>>>, predicate: fn(&T) -> bool, hashset: &mut HashSet<T>, found_nodes: &mut Vec<Rc<RefCell<Node<T>>>>) {
    let children = own.borrow().children.clone();

    if predicate(&own.borrow().value) {
        found_nodes.push(Rc::clone(own));
    }

    for child in children {
        if hashset.contains(&child.borrow().value) {
            continue;
        }

        hashset.insert(child.borrow().value.clone());
        search_in_tree_accumulated_all(&child, predicate, hashset, found_nodes);
    }
}