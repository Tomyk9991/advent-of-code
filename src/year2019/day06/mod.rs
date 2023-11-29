use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use crate::Error;

#[derive(Default, Clone, Debug)]
pub struct Day {
    com: Node
}

#[derive(Default, Clone, Debug)]
pub struct Node {
    value: String,
    children: Vec<Rc<Node>>
}

impl crate::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L", 42)
        ]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        println!("{:#?}", self.com);

        Ok(0)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        todo!()
    }
}

fn create_node(map: &HashMap<&str, Vec<&str>>, name: &str) -> Node {
    return if let Some(pointers) = map.get(name) {
        let mut children = vec![];

        for child_name in pointers {
            children.push(Rc::new(create_node(map, *child_name)));
        }

        Node {
            value: name.to_string(),
            children,
        }
    } else {
        Node {
            value: name.to_string(),
            children: vec![],
        }
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

        let com = create_node(&hashmap, "COM");

        Ok(Self {
            com,
        })
    }
}