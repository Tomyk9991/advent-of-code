use std::str::FromStr;

#[derive(Default, Clone, Debug)]
pub struct Day {
    rotation: Vec<i32>,
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        Ok(Self {
            rotation: s.lines().map(|e| {
                let (dir, val) = &e.split_at(1);
                let val: i32 = val.trim().parse().unwrap_or(0);
                match *dir {
                    "L" => -val,
                    "R" => val,
                    _ => 0
                }
            }).collect(),
        })
    }
}

impl crate::aoc::Day for Day {
    type Output = i32;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82", 3)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![
            ("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82", 6),
             ("R1000", 10)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut dial = 50;

        Ok(self.rotation.iter().fold(0i32, |acc, rot| {
            dial += *rot;
            i32::from(dial % 100 == 0) + acc
        }))
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut dial = 50;
        let mut password = 0;

        for &delta in &self.rotation {
            if delta >= 0 {
                password += (dial + delta) / 100;
            } else {
                let reversed = (100 - dial) % 100;
                password += (reversed - delta) / 100;
            }
            dial = (dial + delta).rem_euclid(100);
        }

        Ok(password)
    }
}