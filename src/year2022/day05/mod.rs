use std::fs;

#[derive(Debug)]
struct Move {
    amount: u32,
    from: u32,
    to: u32
}

fn extract_moves(input: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let second_half = input.split("\r\n\r\n").collect::<Vec<&str>>()[1].split('\n').collect::<Vec<&str>>();

    for move_str in second_half {
        if let [_, amount, _, from, _, to] = move_str.split(' ').collect::<Vec<&str>>()[..] {
            moves.push(Move {
                amount: amount.parse::<u32>().unwrap(),
                from: from.parse::<u32>().unwrap(),
                to: to.trim().parse::<u32>().unwrap(),
            })
        }
    }

    moves
}

fn extract_stack(input: &str) -> Vec<Vec<char>> {
    let first_half = input.split("\r\n\r\n").collect::<Vec<&str>>()[0];
    let reversed = first_half.chars().rev().collect::<String>();

    let lines = reversed.split("\n\r").collect::<Vec<&str>>();
    let amount_stacks = lines[0].split(' ').collect::<Vec<&str>>()[0]
        .parse::<usize>()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(amount_stacks);
    for _ in 0..amount_stacks {
        stacks.push(Vec::new());
    }

    for line in &lines[1..] {
        let mut iterator = line.chars();
        let mut counter: i32 = (amount_stacks - 1) as i32;

        while let Some(_) = iterator.next() {
            let letter = iterator.next().unwrap();
            let _ = iterator.next(); // closing bracket
            let _ = iterator.next(); // space

            if letter != ' ' {
                stacks[counter as usize].push(letter);
            }
            counter -= 1;

            if counter < 0 {
                counter = (amount_stacks - 1) as i32;
            }
        }
    }

    stacks
}

pub struct Day5;

impl crate::year2022::Day for Day5 {
    fn date(&self) -> (i32, i32) {
        (5, 2022)
    }

    fn run(&self) {
        let input = fs::read_to_string("src/year_2022/day5/input.txt").unwrap();

        let mut stack_representation: Vec<Vec<char>> = extract_stack(&input);
        let move_list: Vec<Move> = extract_moves(&input);

        for m in &move_list {
            for _ in 0..m.amount {
                let letter = stack_representation[(m.from - 1) as usize].pop().unwrap();
                stack_representation[(m.to - 1) as usize].push(letter);
            }
        }

        let mut final_word: String = String::from("");
        for stack in &stack_representation {
            let top_letter = stack.last().unwrap();
            final_word += top_letter.to_string().as_str();
        }

        println!("First part: {}", final_word);

        let mut stack_representation: Vec<Vec<char>> = extract_stack(&input);

        for m in &move_list {
            let mut sub_stack = String::from("");
            for _ in 0..m.amount {
                let letter = stack_representation[(m.from - 1) as usize].pop().unwrap();
                sub_stack += letter.to_string().as_str();
            }

            let rev_sub_stack = sub_stack.chars().rev().collect::<String>();

            for crate_str in rev_sub_stack.chars() {
                stack_representation[(m.to - 1) as usize].push(crate_str);
            }
        }

        let mut final_word: String = String::from("");
        for stack in &stack_representation {
            let top_letter = stack.last().unwrap();
            final_word += top_letter.to_string().as_str();
        }

        println!("Second part: {}", final_word);
    }
}