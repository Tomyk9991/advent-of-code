use std::ops::Range;
use std::str::FromStr;

#[derive(Default, Clone, Debug)]
pub struct Day {
    id_ranges: Vec<Range<u64>>,
    ids: Vec<u64>,
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id_ranges: Vec<(u64, u64)> = vec![];
        let mut ids = vec![];
        let mut had_blank = false;
        let mut range = 0..0;

        for line in s.lines() {
            if line.trim().is_empty() {
                had_blank = true;
                continue;
            }

            if had_blank {
                ids.push(line.trim().parse::<u64>()?);
                continue;
            } else {
                let parts = line.trim().split('-').collect::<Vec<_>>();
                id_ranges.push((parts[0].parse::<u64>()?, parts[1].parse::<u64>()?));
            }
        }

        let mut merged = Vec::new();
        id_ranges.sort_unstable();
        ids.sort_unstable();

        for (from, to) in id_ranges {
            if from < range.end {
                range.end = range.end.max(to + 1);
            } else {
                merged.push(range);
                range = from..to + 1;
            }
        }

        merged.push(range);

        Ok(Self { id_ranges: merged, ids })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
            3,
        )]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
            14,
        )]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let position = |id: u64| self.ids.binary_search(&id).unwrap_or_else(|e| e) as u64;
        Ok(self.id_ranges.iter().map(|range| position(range.end) - position(range.start)).sum())
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        Ok(self.id_ranges.iter().map(|range| range.end - range.start).sum())
    }
}
