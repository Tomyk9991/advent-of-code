pub fn a_star<F, G, H, Neighbour>(start: StateDirection, goal: (usize, usize), h: H, g: G, f: F, neighbour: Neighbour) -> Option<(Vec<StateDirection>, i32)>
where
    F: Fn(&StateDirection, i32, i32) -> i32,
    G: Fn(&StateDirection, &StateDirection) -> i32,
    H: Fn(&StateDirection, (usize, usize)) -> i32,
    Neighbour: Fn(&StateDirection) -> Vec<StateDirection>,
{
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<StateDirection, StateDirection> = HashMap::new();
    let mut g_scores = HashMap::new();

    g_scores.insert(start.clone(), 0);
    open_set.push(Node {
        state: start.clone(),
        f_score: f(&start, 0, h(&start, goal)),
        g_score: 0,
    });

    while let Some(current) = open_set.pop() {
        if current.state.position == goal {
            let mut path = vec![current.state.clone()];
            let mut current_state = &current.state;
            while let Some(prev_state) = came_from.get(current_state) {
                path.push(prev_state.clone());
                current_state = prev_state;
            }
            path.reverse();
            return Some((path, current.g_score));
        }

        for neighbor in neighbour(&current.state) {
            let tentative_g_score = g_scores[&current.state] + g(&current.state, &neighbor);

            if tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor.clone(), current.state.clone());
                g_scores.insert(neighbor.clone(), tentative_g_score);
                let h_score = h(&neighbor, goal);
                open_set.push(Node {
                    state: neighbor.clone(),
                    f_score: f(&neighbor, tentative_g_score, h_score),
                    g_score: tentative_g_score,
                });
            }
        }
    }

    None
}

fn neighbours(state: &StateDirection, grid: &Grid<char>) -> Vec<StateDirection> {
    let mut neighbors = Vec::new();
    let (x, y) = state.position;

    if grid.in_bounds(x as isize, y as isize - 1) && grid[(x, y - 1)] != '#' {
        neighbors.push(StateDirection { position: (x, y - 1), direction: state.direction.clone() });
    }
    if grid.in_bounds(x as isize + 1, y as isize) && grid[(x + 1, y)] != '#' {
        neighbors.push(StateDirection { position: (x + 1, y), direction: state.direction.clone() });
    }
    if grid.in_bounds(x as isize, y as isize + 1) && grid[(x, y + 1)] != '#' {
        neighbors.push(StateDirection { position: (x, y + 1), direction: state.direction.clone() });
    }
    if grid.in_bounds(x as isize - 1, y as isize) && grid[(x - 1, y)] != '#' {
        neighbors.push(StateDirection { position: (x - 1, y), direction: state.direction.clone() });
    }

    neighbors
}
pub fn a_star_in_place_grid<F, G, H>(start: StateDirection, goal: (usize, usize), h: H, g: G, f: F, grid: &Grid<char>) -> Option<(Vec<StateDirection>, i32)>
where
    F: Fn(&StateDirection, i32, i32) -> i32,
    G: Fn(&StateDirection, &StateDirection) -> i32,
    H: Fn(&StateDirection, (usize, usize)) -> i32,
{
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<StateDirection, StateDirection> = HashMap::new();
    let mut g_scores = HashMap::new();

    g_scores.insert(start.clone(), 0);
    open_set.push(Node {
        state: start.clone(),
        f_score: f(&start, 0, h(&start, goal)),
        g_score: 0,
    });

    while let Some(current) = open_set.pop() {
        if current.state.position == goal {
            let mut path = vec![current.state.clone()];
            let mut current_state = &current.state;
            while let Some(prev_state) = came_from.get(current_state) {
                path.push(prev_state.clone());
                current_state = prev_state;
            }
            path.reverse();
            return Some((path, current.g_score));
        }

        for neighbor in neighbours(&current.state, grid) {
            let tentative_g_score = g_scores[&current.state] + g(&current.state, &neighbor);

            if tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor.clone(), current.state.clone());
                g_scores.insert(neighbor.clone(), tentative_g_score);
                let h_score = h(&neighbor, goal);
                open_set.push(Node {
                    state: neighbor.clone(),
                    f_score: f(&neighbor, tentative_g_score, h_score),
                    g_score: tentative_g_score,
                });
            }
        }
    }

    None
}

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use crate::utils::grid::Grid;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StateDirection {
    pub position: (usize, usize),
    pub direction: Direction,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq, Debug)]
struct Node<T> {
    state: T,
    f_score: i32,
    g_score: i32,
}

impl<T: Eq + PartialEq> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl<T: Eq + PartialEq> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn a_star_all_paths<F, G, H, Neighbour>(
    start: StateDirection,
    goal: (usize, usize),
    h: H,
    g: G,
    f: F,
    neighbour: Neighbour,
) -> Option<(Vec<Vec<StateDirection>>, i32)>
where
    F: Fn(&StateDirection, i32, i32) -> i32,
    G: Fn(&StateDirection, &StateDirection) -> i32,
    H: Fn(&StateDirection, (usize, usize)) -> i32,
    Neighbour: Fn(&StateDirection) -> Vec<StateDirection>,
{
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<StateDirection, Vec<StateDirection>> = HashMap::new();
    let mut g_scores = HashMap::new();

    g_scores.insert(start.clone(), 0);
    open_set.push(Node {
        state: start.clone(),
        f_score: f(&start, 0, h(&start, goal)),
        g_score: 0,
    });

    while let Some(current) = open_set.pop() {
        if current.state.position == goal {
            let mut all_paths = Vec::new();
            let mut stack = vec![(vec![current.state.clone()], current.state.clone())];

            // Rekonstruiere alle Pfade
            while let Some((mut path, state)) = stack.pop() {
                if let Some(predecessors) = came_from.get(&state) {
                    for pred in predecessors {
                        let mut new_path = path.clone();
                        new_path.push(pred.clone());
                        stack.push((new_path, pred.clone()));
                    }
                } else {
                    path.reverse();
                    all_paths.push(path);
                }
            }

            return Some((all_paths, current.g_score));
        }

        for neighbor in neighbour(&current.state) {
            let tentative_g_score = g_scores[&current.state] + g(&current.state, &neighbor);

            if tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor.clone(), vec![current.state.clone()]);
                g_scores.insert(neighbor.clone(), tentative_g_score);
                let h_score = h(&neighbor, goal);
                open_set.push(Node {
                    state: neighbor.clone(),
                    f_score: f(&neighbor, tentative_g_score, h_score),
                    g_score: tentative_g_score,
                });
            } else if tentative_g_score == *g_scores.get(&neighbor).unwrap_or(&i32::MAX) {
                if let Some(pred_list) = came_from.get_mut(&neighbor) {
                    pred_list.push(current.state.clone());
                }
            }
        }
    }

    None
}
