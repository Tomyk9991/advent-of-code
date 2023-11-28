use std::cmp::Ordering;
use std::fs;

pub struct Day13;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListElement {
    List(List),
    Integer(u8)
}

impl PartialOrd for ListElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.partial_cmp(b),
            (Self::List(a), Self::List(b)) => a.partial_cmp(b),
            (num_a, Self::List(b)) => {
                let a = List { elements: vec![num_a.clone()] };
                a.partial_cmp(b)
            }
            (Self::List(a), num_b) => {
                let b = List { elements: vec![num_b.clone()] };
                a.partial_cmp(&b)
            }
        }
    }
}

impl Ord for ListElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct List {
    elements: Vec<ListElement>
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (a, b) in self.elements.iter().zip(other.elements.iter()) {
            match a.cmp(b) {
                Ordering::Less => return Some(Ordering::Less),
                Ordering::Greater => return Some(Ordering::Greater),
                _ => continue,
            }
        }
        Some(self.elements.len().cmp(&other.elements.len()))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


// impl Part

impl List {
    pub fn empty() -> Self {
        Self { elements: vec![] }
    }


    pub fn from(list_str: &str) -> Self {
        let mut list = List::empty();

        let mut splits = vec![];
        let mut indent_level = 0;
        for (i, char) in list_str.chars().enumerate() {
            match char {
                '[' => indent_level += 1,
                ']' => indent_level -= 1,
                ',' => {
                    if indent_level == 1 {
                        splits.push(i);
                    }
                },
                _ => { }
            }
        }

        splits.push(list_str.len() - 1);

        let mut split: Vec<&str> = vec![];
        let mut current_start = 1;
        for split_index in &splits {
            split.push(&list_str[current_start..*split_index]);
            current_start = split_index + 1;
        }

        for element in split {
            if element.starts_with('[') {
                list.elements.push(ListElement::List(List::from(element)))
            } else if !element.is_empty() {
                list.elements.push(ListElement::Integer(element.parse::<u8>().unwrap()));
            }
        }

        list
    }
}

impl crate::year2022::Day for Day13 {
    fn date(&self) -> (i32, i32) { (13, 2022) }

    fn run(&self) {
        let input = fs::read_to_string("src/year_2022/day13/input.txt").unwrap();
        let pairs: Vec<&str> = input.split("\r\n\r\n").collect();
        let mut lists = vec![];
        let mut sum = 0;
        for (index, line) in pairs.iter().enumerate() {
            let l: Vec<&str> = line.lines().collect();
            let list_one_str = l[0].trim();
            let list_two_str = l[1].trim();

            let list_one = List::from(list_one_str);
            let list_two = List::from(list_two_str);

            if list_one < list_two { // right order
                sum += index + 1;
            }

            lists.push(list_one);
            lists.push(list_two);
        }

        println!("Part one:\t\t{}", sum);


        let dp_1 = List { elements: vec![ListElement::List(List { elements: vec![ListElement::Integer(2)] })] };
        let dp_2 = List { elements: vec![ListElement::List(List { elements: vec![ListElement::Integer(6)] })] };
        lists.push(dp_1.clone());
        lists.push(dp_2.clone());
        lists.sort();

        let dp_1_index = lists.iter().position(|l| *l == dp_1).unwrap() + 1;
        let dp_2_index = lists.iter().position(|l| *l == dp_2).unwrap() + 1;


        println!("Part two:\t\t{}", dp_1_index * dp_2_index);
    }
}