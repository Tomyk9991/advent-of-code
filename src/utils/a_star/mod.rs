use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

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

/// A\* algorithm implementation to find the shortest path from a start state to a goal state.
///
/// # Parameters
/// - `start`: The initial state from which the search begins.
/// - `goal`: The target state that the search aims to reach.
/// - `h`: A heuristic function that estimates the cost from a given state to the goal state.
/// - `g`: A function that calculates the cost to move from one state to another.
/// - `f`: A function that combines the g\_cost and h\_cost to produce the f\_score.
/// - `neighbour`: A function that generates the neighboring states from a given state.
///
/// # Returns
/// An `Option` containing a tuple with a vector of states representing the path from start to goal and the total cost of that path.
/// Returns `None` if no path is found.
pub fn a_star<State, F, G, H, Neighbour>(start: State, goal: State, h: H, g: G, f: F, neighbour: Neighbour) -> Option<(Vec<State>, i32)>
where
    State: Clone + Eq + PartialEq + std::hash::Hash,
    F: Fn(&State, i32, i32) -> i32,
    G: Fn(&State, &State) -> i32,
    H: Fn(&State, &State) -> i32,
    Neighbour: Fn(&State) -> Vec<State>,
{
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<State, State> = HashMap::new();
    let mut g_scores = HashMap::new();

    g_scores.insert(start.clone(), 0);
    open_set.push(Node {
        state: start.clone(),
        f_score: f(&start, 0, h(&start, &goal)),
        g_score: 0,
    });

    while let Some(current) = open_set.pop() {
        if current.state == goal {
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
                let h_score = h(&neighbor, &goal);
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

/// A\* algorithm implementation to find all the shortest path from a start state to a goal state.
///
/// # Parameters
/// - `start`: The initial state from which the search begins.
/// - `goal`: The target state that the search aims to reach.
/// - `h`: A heuristic function that estimates the cost from a given state to the goal state.
/// - `g`: A function that calculates the cost to move from one state to another.
/// - `f`: A function that combines the g\_cost and h\_cost to produce the f\_score.
/// - `neighbour`: A function that generates the neighboring states from a given state.
///
/// # Returns
/// An `Option` containing a tuple with a vector of states representing the path from start to goal and the total cost of that path.
/// Returns `None` if no path is found.
pub fn a_star_all_paths<State, F, G, H, Neighbour>(start: State, goal: State, h: H, g: G, f: F, neighbour: Neighbour) -> Option<(Vec<Vec<State>>, i32)>
where
    State: Clone + Eq + PartialEq + std::hash::Hash,
    F: Fn(&State, i32, i32) -> i32,
    G: Fn(&State, &State) -> i32,
    H: Fn(&State, &State) -> i32,
    Neighbour: Fn(&State) -> Vec<State>,
{
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<State, Vec<State>> = HashMap::new();
    let mut g_scores = HashMap::new();

    g_scores.insert(start.clone(), 0);
    open_set.push(Node {
        state: start.clone(),
        f_score: f(&start, 0, h(&start, &goal)),
        g_score: 0,
    });

    while let Some(current) = open_set.pop() {
        if current.state == goal {
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
                let h_score = h(&neighbor, &goal);
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