use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct Day {
    lines: Vec<Vec<u8>>,
    width: usize,
    start: usize
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().map(str::as_bytes).map(|a| a.to_vec()).collect::<Vec<_>>();
        let width = lines[0].len();
        let start = lines[0].iter().position(|&b| b == b'S').ok_or(crate::aoc::Error::Parse("No start position found".to_string()))?;

        Ok(Day { lines, width, start } )
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............", 21)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............", 40)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        Ok(math(self).0)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        Ok(math(self).1)
    }
}

fn math(day: &mut Day) -> (u64, u64)  {
    let mut splits = 0;
    let mut current = vec![0; day.width];
    let mut next = vec![0i64; day.width];

    current[day.start] = 1;

    for row in &day.lines {
        for (i, &count) in current.iter().enumerate() {
            if count > 0 {
                if row[i] == b'^' {
                    splits += 1;

                    if i > 0 {
                        next[i - 1] += count;
                    }
                    if i < day.width - 1 {
                        next[i + 1] += count;
                    }
                } else {
                    next[i] += count;
                }
            }
        }

        (current, next) = (next, current);
        next.fill(0);
    }

    (splits, current.iter().map(|s| *s as u64).sum())
}