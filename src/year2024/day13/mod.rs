use std::str::FromStr;

#[derive(Default, Clone, Debug)]
struct PriceConfiguration {
    button_a: (f64, f64),
    button_b: (f64, f64),
    prize: (f64, f64)
}

#[derive(Default, Clone)]
pub struct Day {
    price_configurations: Vec<PriceConfiguration>
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = Vec::new();
        let mut current_block = Vec::new();

        for line in s.lines() {
            if line.trim().is_empty() {
                blocks.push(current_block);
                current_block = Vec::new();
            } else {
                current_block.push(line);
            }
        }
        blocks.push(current_block);

        let price_configs = blocks.iter().map(|lines| {
            let mut current_configuration = PriceConfiguration::default();
            if let ["Button", "A:", pos_x, pos_y] = &lines[0].split(" ").collect::<Vec<&str>>()[..] {
                current_configuration.button_a = (pos_x.replace("X+", "").replace(",", "").parse().unwrap_or(0.0), pos_y.replace("Y+", "").parse().ok().unwrap_or(0.0));
            }

            if let ["Button", "B:", pos_x, pos_y] = &lines[1].split(" ").collect::<Vec<&str>>()[..] {
                current_configuration.button_b = (pos_x.replace("X+", "").replace(",", "").parse().ok().unwrap_or(0.0), pos_y.replace("Y+", "").parse().ok().unwrap_or(0.0));
            }

            if let ["Prize:", pos_x, pos_y] = &lines[2].split(" ").collect::<Vec<&str>>()[..] {
                current_configuration.prize = (pos_x.replace("X=", "").replace(",", "").parse().ok().unwrap_or(0.0), pos_y.replace("Y=", "").parse().ok().unwrap_or(0.0));
            }

            current_configuration
        }).collect::<Vec<_>>();

        Ok(Self { price_configurations: price_configs })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#, 480)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#, 875318608908)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        Ok(self.price_configurations.iter().filter_map(|price_config| {
            solve(price_config.button_a.0, price_config.button_a.1, price_config.button_b.0, price_config.button_b.1, price_config.prize.0, price_config.prize.1)
        }).sum())
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        Ok(self.price_configurations.iter().filter_map(|price_config| {
            solve(price_config.button_a.0, price_config.button_a.1, price_config.button_b.0, price_config.button_b.1, (price_config.prize.0 as u64 + 10000000000000) as f64, (price_config.prize.1 as u64 + 10000000000000) as f64)
        }).sum())
    }
}

// Zwei Gleichungen mit zwei Unbekannten, das heißt die Mächtigkeit der Lösungsmenge ist 1
// Stelle system auf, multipliziere beide Gleichungen mit p_x, subtrahiere diese und es bleibt übrig
// wiederhole den prozess für die andere Variable
// (a_x * b_y) * x = (p_x * b_y) - (p_y * b_x)
// x1 = (p_x * b_y - p_y * b_x) / (a_x * b_y - a_y * b_x)
//
// (a_x * x_1) + (b_x * x_2) = p_x
// x2 = (p_x - a_x * x_1) / b_x

// es gibt immer genau eine Lösung, aber nur dann brauchbar, wenn actually integer
fn solve(ax: f64, ay: f64, bx: f64, by: f64, px: f64, py: f64) -> Option<u64> {
    let count_a = (px * by - py * bx) / (ax * by - ay * bx);
    let count_b = (px - ax * count_a) / bx;

    if count_a.fract() == 0.0 && count_b.fract() == 0.0 {
        Some((count_a * 3.0 + count_b) as u64)
    } else {
        None
    }
}

