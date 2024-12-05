use std::str::FromStr;

#[derive(Default, Clone, Debug)]
struct Rule {
    lower: i32,
    upper: i32,
}

#[derive(Default, Clone)]
pub struct Day {
    rules: Vec<Rule>,
    updates: Vec<Vec<i32>>,
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = Vec::new();
        let mut updates = Vec::new();
        let mut rules_done = false;

        for line in s.lines() {
            if line == "" {
                rules_done = true;
                continue;
            }

            match rules_done {
                false => {
                    if let [a, b] = line.split('|').collect::<Vec<&str>>().as_slice() {
                        rules.push(Rule { lower: a.parse()?, upper: b.parse()? });
                        continue;
                    }
                }
                true => {
                    updates.push(line.split(',').filter_map(|n| n.parse::<i32>().ok()).collect::<Vec<i32>>());
                    continue;
                }
            }
        }

        Ok(Self { rules, updates })
    }
}

impl crate::aoc::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47", 143)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47", 123)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut sum = 0;
        for update in &self.updates {
            if check_update(update, &self.rules) {
                let middle_index = update.len() / 2;
                let middle = update[middle_index];
                sum += middle;
            }
        }

        Ok(sum)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut sum = 0;
        for update in &mut self.updates {
            if !check_update(update, &self.rules) {
                let k = re_order(update, &self.rules);
                let middle_index = k.len() / 2;
                let middle = k[middle_index];
                sum += middle;
            }
        }

        Ok(sum)
    }
}

fn check_update(update: &[i32], rules: &[Rule]) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if i == j {
                continue;
            }

            let a = update[i];
            let b = update[j];

            if rules.iter().any(|rule| rule.lower == b && rule.upper == a) {
                return false;
            }
        }
    }

    true
}

fn re_order(update: &[i32], rules: &[Rule]) -> Vec<i32> {
    let mut order = update.iter().map(|a| *a).collect::<Vec<i32>>();

    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if i == j {
                continue;
            }

            let a = update[i];
            let b = update[j];

            if rules.iter().any(|rule| rule.lower == b && rule.upper == a) {
                order.swap(i, j);
            }
        }
    }

    if !check_update(&order, rules) {
        order = re_order(&order, rules);
    }

    order
}