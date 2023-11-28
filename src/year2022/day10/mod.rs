use std::fs;

pub struct Day10;

impl crate::year2022::Day for Day10 {
    fn date(&self) -> (i32, i32) { (10, 2022) }

    fn run(&self) {
        // let input = fs::read_to_string("src/year_2022/day10/test.txt").unwrap();
        let input = fs::read_to_string("src/year_2022/day10/input.txt").unwrap();
        let vec = input.split('\n').collect::<Vec<&str>>();

        let (_, signal_strength) = run_until(&vec, None);
        println!("Part one: {}", signal_strength);

        println!("Part two:");

        for y in 0..6 {
            let mut row: String = String::from("");
            for x in 0..40 {
                let cycle = y * 40 + x;
                let c = if (x - run_until(&vec, Some(cycle)).0).abs() <= 1 { "#" } else { "." };
                row += c;
            }

            println!("{row}");
        }
    }
}

fn run_until(vec: &Vec<&str>, limit: Option<i32>) -> (i32, i32) {
    let target_cycle: Vec<i32> = (20..=220).step_by(40).collect::<Vec<i32>>();
    let mut cycles = 0;
    let mut register_x = 1;
    let mut signal_strength = 0;

    for line in vec {
        match line.split(' ').map(|p| p.trim()).collect::<Vec<&str>>()[..] {
            ["noop"] => {
                cycles += 1;

                signal_strength = increase_st(signal_strength, check_cycle(&target_cycle, cycles, register_x));

                if limit.is_some() && cycles >= limit.unwrap() {
                    break;
                }
            },
            ["addx", unit] => {
                cycles += 1;

                signal_strength = increase_st(signal_strength, check_cycle(&target_cycle, cycles, register_x));
                if limit.is_some() && cycles >= limit.unwrap() {
                    break;
                }
                cycles += 1;
                signal_strength = increase_st(signal_strength, check_cycle(&target_cycle, cycles, register_x));

                register_x += unit.trim().parse::<i32>().unwrap();
                if limit.is_some() && cycles >= limit.unwrap() {
                    break;
                }
            },
            _ => { }
        }
    }

    (register_x, signal_strength)
}

fn increase_st(aggregation: i32, signal_strength: Option<i32>) -> i32 {
    if let Some(strength) = signal_strength {
        return aggregation + strength;
    }

    aggregation
}

fn check_cycle(target_cycle: &[i32], current_cycle: i32, register: i32) -> Option<i32> {
    if target_cycle.contains(&current_cycle) {
        return Some(current_cycle * register);
    }

    None
}
