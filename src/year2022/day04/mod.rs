use std::fs;
use std::ops::Range;
fn range_from_str(target: &str) -> Range<u32> {
    if let [lower, upper] = target.split('-').collect::<Vec<&str>>()[..] {
        return lower.parse::<u32>().unwrap()..upper.trim().parse::<u32>().unwrap()
    }

    0..0
}

fn range_touches(a: &Range<u32>, b: &Range<u32>) -> bool {
    let s = a.start..a.end + 1;
    s.contains(&b.start) || s.contains(&b.end)
}

fn range_contains(a: &Range<u32>, b: &Range<u32>) -> bool {
    let s = a.start..a.end + 1;
    s.contains(&b.start) && s.contains(&b.end)
}

pub struct Day4;

impl crate::year2022::Day for Day4 {
    fn date(&self) -> (i32, i32) {
        (4, 2022)
    }

    fn run(&self) {
        let input = fs::read_to_string("src/year2022/day4/input.txt")
            .unwrap();

        let assignment_pairs_first = input.split('\n')
            .map(|assignment_pair| {
                if let [first, second] = assignment_pair.split(',').collect::<Vec<&str>>()[..] {
                    let r_1 = range_from_str(first);
                    let r_2 = range_from_str(second);

                    return if range_contains(&r_1, &r_2) || range_contains(&r_2, &r_1) {
                        1
                    } else {
                        0
                    }
                }

                0
            }).sum::<u32>();

        let assignment_pairs_second = input.split('\n')
            .map(|assignment_pair| {
                if let [first, second] = assignment_pair.split(',').collect::<Vec<&str>>()[..] {
                    let r_1 = range_from_str(first);
                    let r_2 = range_from_str(second);

                    return if range_touches(&r_1, &r_2) || range_contains(&r_2, &r_1) {
                        1
                    } else {
                        0
                    }
                }

                0
            }).sum::<u32>();

        println!("Amount overlaps: {}", assignment_pairs_first);
        println!("Amount touches: {}", assignment_pairs_second);
    }
}