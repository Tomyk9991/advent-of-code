use std::fs;
fn find_common_char(first: &str, second: &str) -> Option<char> {
    for char in first.chars() {
        if let Some(index) = second.find(char) {
            return Some(second.chars().nth(index).unwrap());
        }
    }

    None
}

fn find_common_char_third(first: &str, second: &str, third: &str) -> Option<char> {
    for char in first.chars() {
        if second.find(char).is_some() {
            if let Some(index) = third.find(char) {
                return Some(third.chars().nth(index).unwrap());
            }
        }
    }

    None
}

pub struct Day3;

impl crate::year2022::Day for Day3 {
    fn date(&self) -> (i32, i32) { (3, 2022) }

    fn run(&self) {
        let input = fs::read_to_string("src/year_2022/day3/input.txt")
            .unwrap();

        let first_part_sum = input.split('\n')
            .map(|rucksack| {
                let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);
                if let Some(common_type) = find_common_char(first_half, second_half) {
                    return if common_type.is_uppercase() { common_type as u32 - 65 + 27 } else { common_type as u32 - 96 };
                }

                0
            }).collect::<Vec<u32>>()
            .iter()
            .sum::<u32>();

        let mut group = vec![];
        let second_part_sum = input.split('\n')
            .map(|rucksack| {
                group.push(rucksack);
                if group.len() == 3 {
                    if let [first, second, third] = group[..] {
                        if let Some(common_type) = find_common_char_third(first, second, third) {
                            let result = if common_type.is_uppercase() { common_type as u32 - 65 + 27 } else { common_type as u32 - 96 };
                            group.clear();
                            return result;
                        }
                    }
                    0
                } else {
                    0
                }
            }).collect::<Vec<u32>>()
            .iter()
            .sum::<u32>();

        println!("Sum: {}", first_part_sum);
        println!("Sum: {}", second_part_sum);
    }
}