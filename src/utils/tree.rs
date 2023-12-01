use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};

#[derive(Default, Clone)]
pub struct Node<T> {
    pub value: T,
    pub parent: Option<Weak<RefCell<Node<T>>>>,
    pub children: Vec<Rc<RefCell<Node<T>>>>,
}


impl<T: Clone + PartialEq> Node<T> {
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

    pub(crate) fn search_in_tree(&self, value: &T) -> Option<Rc<RefCell<Node<T>>>> {
        let children = self.children.clone();

        for child in children {
            if &child.borrow().value == value {
                return Some(child);
            }

            if let Some(found) = child.borrow().search_in_tree(value) {
                return Some(found);
            }
        }

        None
    }
}