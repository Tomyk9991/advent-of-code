use std::str::FromStr;

#[derive(Default, Clone, Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[derive(Default, Clone)]
pub struct Day {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
    time: i32,
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.lines().map(|line| {
            let mut parts = line.split(&['=', ' ', ','][..]).collect::<Vec<_>>();

            let position = (parts[1], parts[2]);
            let velocity = (parts[4], parts[5]);

            Robot {
                position: (position.0.parse().unwrap_or(0), position.1.parse().unwrap_or(0)),
                velocity: (velocity.0.parse().unwrap_or(0), velocity.1.parse().unwrap_or(0)),
            }
        }).collect();

        Ok(Day {
            robots: input,
            time: 100,
            width: 11,
            height: 7,
        })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#, 12)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![]
    }

    fn after_test_1(&mut self) {
        self.time = 100;
        self.width = 101;
        self.height = 103;
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut results = vec![];
        for robot in &self.robots {
            let mut new_position = ((robot.position.0 + robot.velocity.0 * self.time) % self.width, (robot.position.1 + robot.velocity.1 * self.time) % self.height);
            if new_position.0 < 0 {
                new_position.0 += self.width;
            }
            if new_position.1 < 0 {
                new_position.1 += self.height;
            }

            results.push(new_position);
        }

        let vertical_mid = (self.height - 1) / 2;
        let horizontal_mid = (self.width - 1) / 2;

        let mut quads = vec![0; 4];
        for (x, y) in results {
            if x == horizontal_mid || y == vertical_mid {
                continue;
            }

            if x < horizontal_mid {
                if y < vertical_mid {
                    quads[0] += 1;
                } else {
                    quads[1] += 1;
                }
            } else {
                if y < vertical_mid {
                    quads[2] += 1;
                } else {
                    quads[3] += 1;
                }
            }
        }

        Ok(quads.iter().fold(1, |acc, x| acc * x) as u64)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut minimum_safety_factor = u64::MAX;
        let mut best_iteration = 0;

        for time in 0..(self.width * self.height) {
            let mut results = vec![];
            for robot in &self.robots {
                let mut new_position = ((robot.position.0 + robot.velocity.0 * time) % self.width, (robot.position.1 + robot.velocity.1 * time) % self.height);
                if new_position.0 < 0 {
                    new_position.0 += self.width;
                }
                if new_position.1 < 0 {
                    new_position.1 += self.height;
                }

                results.push(new_position);
            }

            let vertical_mid = (self.height - 1) / 2;
            let horizontal_mid = (self.width - 1) / 2;

            let mut quads = vec![0; 4];
            for (x, y) in results {
                if x == horizontal_mid || y == vertical_mid {
                    continue;
                }

                if x < horizontal_mid {
                    if y < vertical_mid {
                        quads[0] += 1;
                    } else {
                        quads[1] += 1;
                    }
                } else {
                    if y < vertical_mid {
                        quads[2] += 1;
                    } else {
                        quads[3] += 1;
                    }
                }
            }

            let distribution_factor = quads.iter().fold(1, |acc, x| acc * x) as u64;
            if distribution_factor < minimum_safety_factor {
                minimum_safety_factor = distribution_factor;
                best_iteration = time;
            }
        }


        Ok(best_iteration as u64)
    }
}