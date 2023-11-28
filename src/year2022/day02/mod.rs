use std::fs;

#[derive(PartialEq)]
enum Selection {
    Rock = 1,
    Papers = 2,
    Scissors = 3
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6
}

impl Selection {
    fn convert_from_str(selection: &str) -> Self {
        match selection {
            "A" | "X" => Selection::Rock,
            "B" | "Y" => Selection::Papers,
            "C" | "Z" => Selection::Scissors,
            _ => Selection::Papers
        }
    }

    fn convert_from_opponent_and_outcome(opponent_selection: &Selection, you: &str) -> Selection {
        match opponent_selection {
            Selection::Rock => {
                match you {
                    "X" => Selection::Scissors, // lose
                    "Y" => Selection::Rock, // draw
                    "Z" => Selection::Papers, // win
                    _ => Selection::Papers
                }
            },
            Selection::Papers => {
                match you {
                    "X" => Selection::Rock, // lose
                    "Y" => Selection::Papers, // draw
                    "Z" => Selection::Scissors, // win
                    _ => Selection::Papers
                }
            },
            Selection::Scissors => {
                match you {
                    "X" => Selection::Papers, // lose
                    "Y" => Selection::Scissors, // draw
                    "Z" => Selection::Rock, // win
                    _ => Selection::Papers
                }
            },
        }
    }
}

fn play_game(opponent: &Selection, you: &Selection) -> Outcome {
    if opponent == you {
        return Outcome::Draw;
    }

    if (*you == Selection::Rock && *opponent == Selection::Scissors) 
    || (*you == Selection::Papers && *opponent == Selection::Rock)
    || (*you == Selection::Scissors && *opponent == Selection::Papers) {
        return Outcome::Win;
    }

    Outcome::Lose
}

pub struct Day2;

impl crate::year2022::Day for Day2 {
    fn date(&self) -> (i32, i32) { (2, 2022) }

    fn run(&self) {
        let input = fs::read_to_string("src/year_2022/day2/input.txt")
            .unwrap();

        let outcomes: Vec<(i32, i32)> = input
            .split('\n')
            .map(|strategy| {
                if let [opponent, you] = strategy.split(' ').collect::<Vec<&str>>()[..] {
                    let opponent_selection: Selection = Selection::convert_from_str(opponent);
                    let your_selection: Selection = Selection::convert_from_str(you);
                    let your_selection_2: Selection = Selection::convert_from_opponent_and_outcome(&opponent_selection, you);

                    let outcome = play_game(&opponent_selection, &your_selection) as i32;
                    let outcome_2 = play_game(&opponent_selection, &your_selection_2) as i32;
                    let result = outcome + (your_selection as i32);
                    let result_2 = outcome_2 + (your_selection_2 as i32);
                    return (result, result_2)
                }

                (0, 0)
            }).collect::<Vec<(i32, i32)>>();

        /* first  10404*/
        /* second 10334 */
        let mut sum_1 = 0;
        let mut sum_2 = 0;
        for (outcome_1, outcome_2) in outcomes {
            sum_1 += outcome_1;
            sum_2 += outcome_2;
        }

        println!("{}", sum_1);
        println!("{}", sum_2);
    }
}