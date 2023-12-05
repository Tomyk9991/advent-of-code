use std::str::FromStr;
use crate::aoc::Error;


#[derive(Default, Clone)]
struct Card {
    winning_numbers: Vec<usize>,
    hand: Vec<usize>,
}

#[derive(Default, Clone)]
pub struct Day {
    cards: Vec<Card>,
}

impl crate::aoc::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 13)
        ]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 30)
        ]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut sum = 0;
        for card in &self.cards {
            let winning_numbers = card.winning_numbers.intersect(&card.hand);

            if !winning_numbers.is_empty() {
                sum += 2_i32.pow((winning_numbers.len() - 1) as u32);
            }
        }

        Ok(sum)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut vector = vec![0; self.cards.len()];

        let mut card_number = 1;


        for card in &self.cards {
            let winning_numbers = card.winning_numbers.intersect(&card.hand);

            for i in card_number + 1..=card_number + winning_numbers.len() {
                vector[i - 1] += 1;
            }

            let amount_inner_iterations = vector[card_number - 1];

            if card_number >= self.cards.len() {
                continue;
            }

            for _ in 0..amount_inner_iterations {
                let inner_card = &self.cards[card_number - 1];
                let inner_winning_numbers = inner_card.winning_numbers.intersect(&inner_card.hand);

                for i in card_number + 1..=card_number + inner_winning_numbers.len() {
                    vector[i - 1] += 1;
                }
            }

            card_number += 1;
        }

        let vector = vector.iter().map(|a| a + 1).collect::<Vec<_>>();
        let s = vector.iter().sum::<usize>();
        Ok((s) as i32)
    }
}


trait Intersection<T: std::cmp::Eq + std::clone::Clone> {
    fn intersect(&self, other: &[T]) -> Vec<T>;
}

impl<T: Eq + Clone> Intersection<T> for Vec<T> {
    fn intersect(&self, other: &[T]) -> Vec<T> {
        self.iter().filter(|x| other.contains(x)).cloned()
            .collect::<Vec<T>>()
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<_> = s.lines().filter_map(|line| {
            let r = line
                .split_inclusive(&[':', ' ', '|'])
                .filter(|a| !a.trim().is_empty())
                .map(|a| a.trim())
                .collect::<Vec<_>>();

            if let Some(pipe_index) = r.iter().position(|a| *a == "|") {
                if let ["Card", _card_number, solution @ .., "|"] = &r[..=pipe_index] {
                    if let ["|", hand @ ..] = &r[pipe_index..] {
                        return Some(Card {
                            winning_numbers: solution.iter().map(|a| a.parse::<usize>().unwrap_or(0)).collect(),
                            hand: hand.iter().map(|a| a.parse::<usize>().unwrap_or(0)).collect(),
                        });
                    }
                }
            }

            None
        }).collect();

        Ok(Self {
            cards
        })
    }
}