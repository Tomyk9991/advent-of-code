pub fn a_star<F, G, H, Neighbour>(start: State, goal: (usize, usize), h: H, g: G, f: F, neighbour: Neighbour) -> Option<(Vec<State>, i32)>
where
    F: Fn(&State, i32, i32) -> i32,
    G: Fn(&State, &State) -> i32,
    H: Fn(&State, (usize, usize)) -> i32,
    Neighbour: Fn(&State) -> Vec<State>,
{
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<State, State> = HashMap::new();
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

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct State {
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
struct Node {
    state: State,
    f_score: i32,
    g_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn a_star_all_paths<F, G, H, Neighbour>(
    start: State,
    goal: (usize, usize),
    h: H,
    g: G,
    f: F,
    neighbour: Neighbour,
) -> Option<(Vec<Vec<State>>, i32)>
where
    F: Fn(&State, i32, i32) -> i32,
    G: Fn(&State, &State) -> i32,
    H: Fn(&State, (usize, usize)) -> i32,
    Neighbour: Fn(&State) -> Vec<State>,
{
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<State, Vec<State>> = HashMap::new();
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
