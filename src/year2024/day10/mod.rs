use std::cell::RefCell;
use std::collections::{HashSet};
use std::rc::Rc;
use std::str::FromStr;
use crate::utils::grid::Grid;
use crate::utils::tree::Node;

#[derive(Default, Clone)]
pub struct Day {
    map: Grid<u32>,
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let grid = s.lines()
            .map(|line| {
                width = line.len();
                line.chars().map(|c| c.to_digit(10).unwrap_or(12)).collect::<Vec<u32>>()
            })
            .flatten()
            .collect::<Vec<u32>>();

        let height = s.lines().count();
        Ok(Self { map: Grid::new(width, height, grid) })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#, 36)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#, 81)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let roots = Node::try_from_grid(&self.map, |c| *c == 0, |root, node, _| *node == *root + 1, false);
        let mut visited = HashSet::new();
        let mut counter = 0;

        for root in roots {
            let mut stack = vec![root];
            while let Some(node) = stack.pop() {
                if visited.contains(&node.as_ptr()) {
                    continue;
                }
                visited.insert(node.as_ptr());
                let borrowed = node.borrow();

                if borrowed.value == 9 {
                    counter += 1;
                }
                if !borrowed.children.is_empty() {
                    stack.extend(borrowed.children.iter().map(|c| c.clone()));
                }
            }
        }

        Ok(counter)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let roots = Node::try_from_grid(&self.map, |c| *c == 0, |root, node, _| *node == *root + 1, true);
        let mut distinct_paths = 0;

        for root in roots {
            distinct_paths += count_individual_trails(&root);
        }

        Ok(distinct_paths as u64)
    }
}

fn count_individual_trails(current_node: &Rc<RefCell<Node<u32>>>) -> usize {
    if current_node.borrow().value == 9 {
        return 1;
    }

    let mut total_paths = 0;

    for child in current_node.borrow().children.iter() {
        total_paths += count_individual_trails(child);
    }

    total_paths
}