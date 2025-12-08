use std::str::FromStr;
use crate::utils::grid::{Vec3};

#[derive(Debug, Default, Clone)]
pub struct Day {
    positions: Vec<Vec3>,
    limit: usize
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            positions: s.lines()
                .map(|l| {
                    let coords: Vec<i64> = l
                        .split(',')
                        .map(|n| n.parse::<i64>().map_err(|e| crate::aoc::Error::Parse(e.to_string())))
                        .collect::<Result<Vec<i64>, crate::aoc::Error>>()?;
                    if coords.len() != 3 {
                        return Err(crate::aoc::Error::Parse("Expected 3 coordinates per line".to_string()));
                    }
                    Ok(Vec3::new(coords[0], coords[1], coords[2]))
                }).collect::<Result<Vec<Vec3>, crate::aoc::Error>>()?,
            limit: 10,
        })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689", 40)]
    }

    fn after_test_1(&mut self) {
        self.limit = 1000;
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689", 25272)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let pairs = distances(&self.positions);

        let mut sets: Vec<_> = (0..self.positions.len()).map(|i| vec![i]).collect();
        for (i, j, ..) in pairs.iter().take(self.limit) {
            let first = (0..sets.len()).find(|&k| sets[k].contains(i));
            let second = (0..sets.len()).find(|&k| sets[k].contains(j));
            if let (Some(first), Some(second)) = (first, second) {
                let min = first.min(second);
                let max = first.max(second);

                if min != max {
                    let other = sets.remove(max);
                    sets[min].extend(other);
                }
            }
        }

        let mut sizes: Vec<_> = sets.iter().map(Vec::len).collect();
        sizes.sort_unstable();
        Ok(sizes.iter().rev().take(3).map(|a| *a as u64).product())
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let pairs = distances(&self.positions);
        let mut seen = vec![false; self.positions.len()];
        let mut remaining = self.positions.len();

        for &(i, j, _) in &pairs {
            remaining -= usize::from(!seen[i]) + usize::from(!seen[j]);
            seen[i] = true;
            seen[j] = true;

            if remaining == 0 {
                return Ok((self.positions[i].x * self.positions[j].x) as u64);
            }
        }

        Err(crate::aoc::Error::NoSolutionFound)?
    }
}

fn distances(positions: &[Vec3]) -> Vec<(usize, usize, i64)> {
    let mut pairs = Vec::with_capacity(positions.len() * (positions.len() - 1));
    for (i, &v1) in positions.iter().enumerate() {
        for (j, &v2) in positions.iter().enumerate().skip(i + 1) {
            let distance = v1.distance_squared(&v2);
            pairs.push((i, j, distance));
        }
    }

    pairs.sort_unstable_by_key(|&(_, _, dist)| dist);
    pairs
}