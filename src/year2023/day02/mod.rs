use std::str::FromStr;

use crate::Error;

// R G B = RED GREEN BLUE
type Color = (usize, usize, usize);

#[derive(Default, Clone)]
pub struct Day {
    games: Vec<Vec<Vec<Color>>>
}

impl crate::Day for Day {
    type Output = usize;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 8)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 2286)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let constraint: Color = (12, 13, 14);

        let mut possible_game_indices = vec![];
        for (index, game) in self.games.iter().enumerate() {
            let mut fail_count = 0;
            for set in game {
                for color in set {
                    if color.0 > constraint.0 || color.1 > constraint.1 || color.2 > constraint.2 {
                        fail_count += 1;
                    }
                }
            }

            if fail_count == 0 {
                possible_game_indices.push(index);
            }
        }

        Ok(possible_game_indices.iter().sum::<usize>() + possible_game_indices.len())
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut sum = 0;
        for game in &self.games {
            let mut minimum: Color = (usize::MIN, usize::MIN, usize::MIN);

            for set in game {
                for color in set {
                    if color.0 > minimum.0 {
                        minimum.0 = color.0;
                    }

                    if color.1 > minimum.1 {
                        minimum.1 = color.1;
                    }

                    if color.2 > minimum.2 {
                        minimum.2 = color.2;
                    }
                }
            }

            sum += minimum.0 * minimum.1 * minimum.2;
        }

        Ok(sum)
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let games = s.lines()
            .map(|line| line.split(':').collect::<Vec<_>>()[1])
            .map(|game_data| game_data.split(';').collect::<Vec<_>>())
            .map(|games| games.iter().map(|set| {
                set.split(',')
                    .map(|color| {
                        let s = &color.split(' ').collect::<Vec<_>>()[1..];
                        let color: Color = match s {
                            [color_amount, "red"] =>    (color_amount.parse::<usize>().unwrap_or(0), 0, 0),
                            [color_amount, "green"] =>  (0, color_amount.parse::<usize>().unwrap_or(0), 0),
                            [color_amount, "blue"] =>   (0, 0, color_amount.parse::<usize>().unwrap_or(0)),
                            _ => unreachable!("Parsing didnt match")
                        };
                        color
                    }).collect::<Vec<_>>()
            }).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Self {
            games,
        })
    }
}