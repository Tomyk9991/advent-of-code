use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::str::FromStr;

use crate::aoc::Error;

#[derive(Debug, Clone, Copy)]
enum Part {
    ExtremelyCoolLooking(usize),
    Musical(usize),
    Aerodynamic(usize),
    Shiny(usize),
}

impl Part {
    fn value(&self) -> usize {
        match self {
            Part::ExtremelyCoolLooking(t) => *t,
            Part::Musical(t) => *t,
            Part::Aerodynamic(t) => *t,
            Part::Shiny(t) => *t
        }
    }
}

#[derive(Debug, Clone)]
enum PartDescription {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone)]
enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
enum Condition {
    Conditional {
        part: PartDescription,
        operator: Operator,
        threshold: usize,
        next_rule: String,
    },
    ConditionLess(String),
}

impl Condition {
    fn validate(&self, parts: &[Part; 4]) -> Option<&str> {
        match self {
            Condition::Conditional { part: part_description, operator, threshold, next_rule} => {
                let value = if let (PartDescription::ExtremelyCoolLooking, p) = (part_description, parts[0]) {
                    p.value()
                } else if let (PartDescription::Musical, p) = (part_description, parts[1]) {
                    p.value()
                } else if let (PartDescription::Aerodynamic, p) = (part_description, parts[2]) {
                    p.value()
                } else if let (PartDescription::Shiny, p) = (part_description, parts[3]) {
                    p.value()
                } else {
                    0
                };

                let result = match operator {
                    Operator::LessThan => value < *threshold,
                    Operator::GreaterThan => value > *threshold
                };

                if result { Some(next_rule) } else { None }
            }
            Condition::ConditionLess(a) => Some(a)
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
}

#[derive(Debug, Clone, Default)]
pub struct Day {
    metal_shapes: Vec<[Part; 4]>,
    workflows: HashMap<String, Workflow>,
}


impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("px{a<2006:qkq,m>2090:A,rfg}\r\npv{a>1716:R,A}\r\nlnx{m>1548:A,A}\r\nrfg{s<537:gd,x>2440:R,A}\r\nqs{s>3448:A,lnx}\r\nqkq{x<1416:A,crn}\r\ncrn{x>2662:A,R}\r\nin{s<1351:px,qqz}\r\nqqz{s>2770:qs,m<1801:hdj,R}\r\ngd{a>3333:R,R}\r\nhdj{m>838:A,pv}\r\n\r\n{x=787,m=2655,a=1222,s=2876}\r\n{x=1679,m=44,a=2067,s=496}\r\n{x=2036,m=264,a=79,s=2244}\r\n{x=2461,m=1339,a=466,s=291}\r\n{x=2127,m=1623,a=2188,s=1013}", 19114)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("px{a<2006:qkq,m>2090:A,rfg}\r\npv{a>1716:R,A}\r\nlnx{m>1548:A,A}\r\nrfg{s<537:gd,x>2440:R,A}\r\nqs{s>3448:A,lnx}\r\nqkq{x<1416:A,crn}\r\ncrn{x>2662:A,R}\r\nin{s<1351:px,qqz}\r\nqqz{s>2770:qs,m<1801:hdj,R}\r\ngd{a>3333:R,R}\r\nhdj{m>838:A,pv}\r\n\r\n{x=787,m=2655,a=1222,s=2876}\r\n{x=1679,m=44,a=2067,s=496}\r\n{x=2036,m=264,a=79,s=2244}\r\n{x=2461,m=1339,a=466,s=291}\r\n{x=2127,m=1623,a=2188,s=1013}", 167409079868000)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut result = 0;

        for shape in &self.metal_shapes {
            let mut current = self.workflows.get("in");

            while let Some(workflow) = current {
                for condition in &workflow.conditions {
                    if let Some(validation) = condition.validate(shape) {
                        if validation == "A" {
                            result += shape[0].value();
                            result += shape[1].value();
                            result += shape[2].value();
                            result += shape[3].value();

                            current = None;
                            break;
                        }

                        if validation == "R" {
                            current = None;
                            break;
                        }

                        current = self.workflows.get(validation);
                        break;
                    }
                }
            }
        }

        Ok(result)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut result = 0;
        let mut open = VecDeque::new();

        open.push_back(("in", vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)]));

        while let Some((current, mut ranges)) = open.pop_front() {
            if current == "A" {
                result += ranges.iter()
                    .map(|r| r.1 - r.0 + 1)
                    .product::<usize>();
                continue;
            } else if current == "R" {
                continue;
            }

            struct ConditionalStruct<'a> {
                operator: &'a Operator,
                threshold: usize,
                next_rule: &'a String,
            }

            if let Some(workflow) = self.workflows.get(current) {
                for condition in &workflow.conditions[0..workflow.conditions.len() - 1] {
                    let (index, ConditionalStruct { operator, threshold, next_rule }) = match condition {
                        Condition::Conditional { part, threshold, next_rule, operator} => {
                            match part {
                                PartDescription::ExtremelyCoolLooking => (0, ConditionalStruct { threshold: (*threshold), next_rule, operator }),
                                PartDescription::Musical => (1, ConditionalStruct { threshold: (*threshold), next_rule, operator }),
                                PartDescription::Aerodynamic => (2, ConditionalStruct { threshold: (*threshold), next_rule, operator }),
                                PartDescription::Shiny => (3, ConditionalStruct { threshold: (*threshold), next_rule, operator })
                            }
                        },
                        Condition::ConditionLess(_) => unreachable!()
                    };


                    let mut next_range = ranges.clone();
                    let range = ranges[index];

                    if (range.0..range.1).contains(&threshold) {
                        next_range[index] = match operator {
                            Operator::LessThan => (range.0, threshold - 1),
                            Operator::GreaterThan => (threshold + 1, range.1)
                        };

                        ranges[index] = match operator {
                            Operator::LessThan => (threshold, range.1),
                            Operator::GreaterThan => (range.0, threshold)
                        };

                        open.push_back((next_rule, next_range));
                    }
                }

                match self.workflows[current].conditions.iter().last().unwrap() {
                    Condition::ConditionLess(s) => { open.push_back((s, ranges)); }
                    Condition::Conditional { .. } => unreachable!(),
                }
            }
        }

        Ok(result)
    }
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [name, conditions @ ..] = &s.split(&['{', ',', '}']).collect::<Vec<_>>()[..] {
            return Ok(Self {
                name: name.to_string(),
                conditions: conditions.iter().filter(|c| !c.trim().is_empty()).filter_map(|c| c.parse().ok()).collect(),
            });
        }


        Err(Error::StringParse(s.to_string()))
    }
}

impl FromStr for PartDescription {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Self::ExtremelyCoolLooking,
            "m" => Self::Musical,
            "a" => Self::Aerodynamic,
            "s" => Self::Shiny,
            _ => panic!()
        })
    }
}

impl FromStr for Operator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ">" => Self::GreaterThan,
            "<" => Self::LessThan,
            _ => panic!()
        })
    }
}

impl FromStr for Condition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('<', " < ").replace('>', " > ").replace(':', " : ");
        let s_ref = s.split(' ').collect::<Vec<_>>();
        if let [part, operator, threshold, ":", next_rule] = &s_ref[..] {
            return Ok(Condition::Conditional {
                part: part.parse().unwrap_or(PartDescription::Aerodynamic),
                operator: operator.parse().unwrap_or(Operator::LessThan),
                threshold: threshold.parse::<usize>().unwrap_or(0),
                next_rule: next_rule.to_string(),
            });
        } else if let [next_rule] = &s_ref[..] {
            return Ok(Condition::ConditionLess(next_rule.to_string()));
        }

        Err(Error::StringParse(s))
    }
}


impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let all_lines = s.split("\r\n\r\n").collect::<Vec<_>>();

        Ok(Self {
            workflows: all_lines[0].lines()
                .filter_map(|line| line.parse().ok())
                .map(|a: Workflow| (a.name.to_string(), a.clone()))
                .collect::<HashMap<String, Workflow>>(),
            metal_shapes: all_lines[1].lines().map(|line| {
                let k = line.replace(['{', '}'], "");
                let parts = k.split(',').collect::<Vec<_>>();

                let mut r: [Part; 4] = [Part::Musical(0); 4];

                for (i, part_str) in parts.iter().enumerate() {
                    if let [part, number] = part_str.split('=').collect::<Vec<_>>()[..] {
                        let part = match part {
                            "x" => Part::ExtremelyCoolLooking(number.parse().unwrap_or(0)),
                            "m" => Part::Musical(number.parse().unwrap_or(0)),
                            "a" => Part::Aerodynamic(number.parse().unwrap_or(0)),
                            "s" => Part::Shiny(number.parse().unwrap_or(0)),
                            _ => panic!()
                        };

                        r[i] = part;
                    }
                }

                r
            }).collect(),
        })
    }
}