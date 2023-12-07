use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use itertools::Itertools;

use crate::aoc::Error;

#[derive(Debug, Clone, Default)]
pub struct Day {
    hands: Vec<Hand>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Type {
    A = 14,
    K = 13,
    Q = 12,
    /// to get the correct answer for problem two, replace J = 11, with J = 1
    J = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum HandStrength {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug, Clone)]
struct Hand {
    values: Vec<Type>,
    bid: u32,
    hand_strength: HandStrength,
}

trait AreEqual {
    fn are_equal(&self, amount: usize) -> bool;
    fn are_equal_distinct(&self, amount: usize, distinct_from: &[Type]) -> (bool, Type);
}

impl Hand {
    fn contains(&self, p0: &Type) -> bool {
        self.values.iter().any(|f| f == p0)
    }
    pub fn rebuild(&mut self) {
        let virtual_hand = self.clone();

        let js = virtual_hand.values.iter()
            .enumerate()
            .filter_map(|(i, s)| if *s == Type::J { Some(i) } else { None })
            .collect::<Vec<_>>();

        let max_hand = HandPermutationIterator::new(virtual_hand.clone(), js)
            .map(|m| HandStrength::from(&m.values))
            .max()
            .unwrap_or(self.hand_strength.clone());


        self.hand_strength = max_hand;
    }
}

struct HandPermutationIterator {
    hand: Hand,
    joker_indices: Vec<usize>,
    permutations: VecDeque<Vec<Type>>,
}

impl HandPermutationIterator {
    fn new(hand: Hand, joker_indices: Vec<usize>) -> Self {
        let mut joker_indices = joker_indices;
        joker_indices.sort_unstable();

        let types = vec![
            Type::Two,
            Type::Three,
            Type::Four,
            Type::Five,
            Type::Six,
            Type::Seven,
            Type::Eight,
            Type::Nine,
            Type::Ten,
            Type::J,
            Type::Q,
            Type::K,
            Type::A,
        ];

        let mut replacements = vec![];
        for _ in 0..joker_indices.len() {
            replacements.push(types.clone());
        }

        let permutations = replacements.iter()
            .cloned()
            .multi_cartesian_product()
            .collect::<VecDeque<_>>();

        Self {
            hand,
            joker_indices,
            permutations,
        }
    }
}

impl Iterator for HandPermutationIterator {
    type Item = Hand;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(permutation) = self.permutations.pop_front() {
            for (joker_index, replacement) in self.joker_indices.iter().zip(permutation.iter()) {
                self.hand.values[*joker_index] = replacement.clone();
            }

            Some(self.hand.clone())
        } else {
            None
        }
    }
}


impl AreEqual for &Vec<Type> {
    fn are_equal(&self, amount: usize) -> bool {
        self.are_equal_distinct(amount, &[]).0
    }

    fn are_equal_distinct(&self, amount: usize, distinct_from: &[Type]) -> (bool, Type) {
        let mut hash_map: HashMap<Type, usize> = HashMap::new();

        for ty in self.iter() {
            if let Some(value) = hash_map.get_mut(ty) {
                *value += 1;
            } else {
                hash_map.insert(ty.clone(), 1);
            }
        }

        for (ty, value) in &hash_map {
            if *value == amount && !distinct_from.contains(ty) {
                return (true, ty.clone());
            }
        }

        (false, Type::Two)
    }
}

impl From<&Vec<Type>> for HandStrength {
    fn from(value: &Vec<Type>) -> Self {
        if value.are_equal(5) {
            return HandStrength::FiveOfAKind;
        }

        if value.are_equal(4) {
            return HandStrength::FourOfAKind;
        }

        if value.are_equal(3) && value.are_equal(2) {
            return HandStrength::FullHouse;
        }

        if value.are_equal(3) {
            return HandStrength::ThreeOfAKind;
        }

        let is_two_pair = value.are_equal_distinct(2, &[]);

        if is_two_pair.0 {
            let is_two_pair_second = value.are_equal_distinct(2, &[is_two_pair.1.clone()]);

            return if is_two_pair_second.0 {
                HandStrength::TwoPair
            } else {
                HandStrength::OnePair
            };
        }

        HandStrength::HighCard
    }
}

impl From<char> for Type {
    fn from(value: char) -> Self {
        match value {
            'A' => Type::A,
            'K' => Type::K,
            'Q' => Type::Q,
            'J' => Type::J,
            'T' => Type::Ten,
            '9' => Type::Nine,
            '8' => Type::Eight,
            '7' => Type::Seven,
            '6' => Type::Six,
            '5' => Type::Five,
            '4' => Type::Four,
            '3' => Type::Three,
            '2' => Type::Two,
            _ => Type::A
        }
    }
}


impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_strength.eq(&other.hand_strength) && self.values.eq(&other.values)
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp = self.hand_strength.cmp(&other.hand_strength);

        if cmp == Ordering::Equal {
            for (ty1, ty2) in self.values.iter().zip(&other.values) {
                let type_cmp = ty1.cmp(ty2);

                match type_cmp {
                    Ordering::Equal => continue,
                    t => { return Some(t); }
                }
            }
        }

        Some(cmp)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ord) = self.hand_strength.partial_cmp(&other.hand_strength) {
            return ord;
        } else {
            Ordering::Equal
        }
    }
}


impl crate::aoc::Day for Day {
    type Output = u32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483", 6440)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483", 5905)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut hands = self.hands.clone();
        hands.sort();

        Ok(hands.iter().enumerate().map(|(index, hand)| {
            hand.bid * (index + 1) as u32
        }).sum())
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut hands = self.hands.clone();

        for hand in hands.iter_mut() {
            if hand.contains(&Type::J) {
                hand.rebuild();
            }
        }
        hands.sort();


        Ok(hands.iter().enumerate().map(|(index, hand)| {
            hand.bid * (index + 1) as u32
        }).sum())
    }
}


impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let all_hands = s.lines().map(|line| {
            let mut values = line.split_whitespace();
            let hand = values.next().unwrap_or("");
            let bid = values.next().unwrap_or("");

            (hand.chars().map(Type::from).collect::<Vec<_>>(), bid.parse::<u32>().unwrap_or(0))
        }).collect::<Vec<_>>();

        Ok(Self {
            hands: all_hands
                .iter()
                .map(|a| Hand {
                    values: a.0.clone(),
                    bid: a.1,
                    hand_strength: HandStrength::from(&a.0),
                }).collect::<Vec<_>>(),
        })
    }
}