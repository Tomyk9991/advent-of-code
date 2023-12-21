use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::str::FromStr;

use itertools::Itertools;

use crate::aoc::Error;

#[derive(Debug, Clone, PartialEq)]
enum LogicType {
    /// %
    FlipFlop,
    /// &
    Conjunction,
}

#[derive(Debug, Clone, PartialEq)]
enum Memory {
    Off,
    On(HashMap<String, String>),
    // On Condition less
    OnCL,
}

#[derive(Debug, Clone)]
struct LogicGate {
    name: String,
    ty: LogicType,
    memory: Memory,
    outputs: Vec<String>,
}

impl LogicGate {
    fn new(name: &str, ty: char, outputs: Vec<String>) -> Self {
        let ty = match ty {
            '%' => LogicType::FlipFlop,
            '&' => LogicType::Conjunction,
            _ => unreachable!()
        };

        Self {
            name: name.to_string(),
            memory: match ty {
                LogicType::FlipFlop => Memory::Off,
                LogicType::Conjunction => Memory::On(HashMap::new())
            },
            ty,
            outputs,
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct Day {
    gates: HashMap<String, LogicGate>,
    broadcast_outputs: Vec<String>,
}


impl crate::aoc::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![
            (r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#, 11687500),
            (r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#, 32000000),
        ]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut low_pulses = 0;
        let mut high_pulses = 0;

        // origin, target, pulse
        type Format = (String, String, String);

        for _ in 0..1000 {
            low_pulses += 1;
            // origin, target, pulse
            let mut queue: VecDeque<Format> = VecDeque::from_iter(self.broadcast_outputs.iter().map(|x| ("broadcaster".to_string(), x.to_string(), "lo".to_string())));

            while let Some((origin, target, pulse)) = queue.pop_front() {
                if pulse == "lo" {
                    low_pulses += 1;
                } else {
                    high_pulses += 1;
                }

                if let Some(gate) = self.gates.get_mut(&target) {
                    if gate.ty == LogicType::FlipFlop {
                        if pulse == "lo" {
                            gate.memory = if gate.memory == Memory::Off { Memory::OnCL } else { Memory::Off };
                            let new_pulse = match gate.memory {
                                Memory::OnCL => String::from("hi"),
                                Memory::Off => String::from("lo"),
                                Memory::On(_) => unreachable!()
                            };

                            // send pulse

                            for output in &gate.outputs {
                                queue.push_back((gate.name.to_string(), output.to_string(), new_pulse.to_string()))
                            }
                        }
                    } else {
                        match gate.memory {
                            Memory::On(ref mut hashmap) => {
                                hashmap.insert(origin.to_string(), pulse);

                                let new_pulse = match hashmap.values().all(|a| a == "hi") {
                                    true => "lo".to_string(),
                                    false => "hi".to_string()
                                };

                                for output in &gate.outputs {
                                    queue.push_back((gate.name.to_string(), output.to_string(), new_pulse.to_string()))
                                }
                            }
                            _ => unreachable!()
                        }
                    }
                } else {
                    continue;
                }
            }
        }


        Ok(low_pulses * high_pulses)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        if self.gates.len() == 0 || self.broadcast_outputs.len() == 0 {
            return Ok(0);
        }
        // find every name where rx is in output
        let feed = self.gates.iter().filter_map(|(name, gate)| {
            if gate.outputs.contains(&String::from("rx")) { Some(name.to_string()) } else { None }
        }).collect::<Vec<String>>()[0].clone();


        // idee: Alle auf rx zeigende gatter müssen true pulsieren, damit das ebenfalls wahr ergibt, da auf rx mit einem Conjunction zeigt wird
        // Suche gemeinsame Looplänge, wo alle gleich pulsieren

        let mut cycle_lengths = HashMap::new();
        let mut seen: HashMap<String, usize> = HashMap::from_iter(self.gates.iter().filter_map(|(name, gate)| {
            if gate.outputs.contains(&feed) { Some((name.to_string(), 0)) } else { None }
        }));
        let mut presses = 0;

        // origin, target, pulse
        type Format = (String, String, String);
        let mut result = 0;

        'outer: loop {
            presses += 1;
            // origin, target, pulse
            let mut queue: VecDeque<Format> = VecDeque::from_iter(self.broadcast_outputs.iter().map(|x| ("broadcaster".to_string(), x.to_string(), "lo".to_string())));

            while let Some((origin, target, pulse)) = queue.pop_front() {
                if let Some(gate) = self.gates.get_mut(&target) {
                    if gate.name == feed && pulse == "hi" {
                        if let Some(value) = seen.get_mut(&origin) {
                            *value += 1;
                        }

                        if !cycle_lengths.contains_key(&origin) {
                            cycle_lengths.insert(origin.to_string(), presses.clone());
                        }

                        if seen.values().into_iter().all(|a| *a > 0) {
                            let mut x = 1;

                            for cycle_length in cycle_lengths.values() {
                                x = x * cycle_length / gcd(x, *cycle_length);
                            }
                            result = x;
                            break 'outer;
                        }
                    }

                    if gate.ty == LogicType::FlipFlop {
                        if pulse == "lo" {
                            gate.memory = if gate.memory == Memory::Off { Memory::OnCL } else { Memory::Off };
                            let new_pulse = match gate.memory {
                                Memory::OnCL => String::from("hi"),
                                Memory::Off => String::from("lo"),
                                Memory::On(_) => unreachable!()
                            };

                            // send pulse
                            for output in &gate.outputs {
                                queue.push_back((gate.name.to_string(), output.to_string(), new_pulse.to_string()))
                            }
                        }
                    } else {
                        match gate.memory {
                            Memory::On(ref mut hashmap) => {
                                hashmap.insert(origin.to_string(), pulse);

                                let new_pulse = match hashmap.values().all(|a| a == "hi") {
                                    true => "lo".to_string(),
                                    false => "hi".to_string()
                                };

                                for output in &gate.outputs {
                                    queue.push_back((gate.name.to_string(), output.to_string(), new_pulse.to_string()))
                                }
                            }
                            _ => unreachable!()
                        }
                    }
                } else {
                    continue;
                }
            }
        }


        Ok(result)
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut broadcast_outputs = vec![];
        let mut gates = HashMap::new();

        for line in s.lines() {
            if let [left, right] = &line.trim().split(" -> ").collect::<Vec<_>>()[..] {
                let outputs = right.split(", ").map(|a| a.to_string()).collect::<Vec<_>>();

                if *left == "broadcaster" {
                    broadcast_outputs = outputs;
                } else {
                    let ty = left.chars().nth(0).unwrap_or(' ');
                    let name = &left[1..];

                    gates.insert(name.to_string(), LogicGate::new(name, ty, outputs));
                }
            };
        }

        let mut populate = vec![];

        for (name, gate) in &gates {
            for output in &gate.outputs {
                if let Some(m) = gates.get(output) {
                    if m.ty == LogicType::Conjunction {
                        populate.push((output.to_string(), name.to_string(), "lo".to_string()));
                    }
                }
            }
        }

        for p in populate {
            if let Some(gate) = gates.get_mut(&p.0) {
                match gate.memory {
                    Memory::On(ref mut hashmap) => { hashmap.insert(p.1, p.2); }
                    _ => {}
                }
            }
        }

        Ok(Self {
            gates,
            broadcast_outputs,
        })
    }
}