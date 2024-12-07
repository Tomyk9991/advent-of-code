use std::str::FromStr;

#[derive(Default, Clone, Debug)]
struct Equation {
    result: u64,
    parts: Vec<u64>,
}

impl FromStr for Equation {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts_str = s.split(" ");
        let result = parts_str
            .next()
            .ok_or(Self::Err::StringParse("Cannot parse".to_string()))?
            .replace(":", "")
            .parse::<u64>()?;

        let parts = parts_str
            .map(|part| part.trim().parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            result,
            parts,
        })
    }
}

#[derive(Default, Clone)]
pub struct Day {
    equations: Vec<Equation>,
    operations: Vec<fn(u64, u64) -> u64>,
}

impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let equations = s
            .lines()
            .map(|line| line.parse::<Equation>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { equations, operations: vec![|a, b| a + b, |a, b| a * b] })
    }
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#, 3749)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#, 11387)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        Ok(self.equations
            .iter()
            .filter(|eq| self.can_solve_equation(eq))
            .map(|eq| eq.result)
            .sum()
        )
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        self.operations.push(|a, b| {
            let digits = (b as f64).log10().floor() as u32 + 1;
            a * 10u64.pow(digits) + b
        });

        Ok(self.equations
            .iter()
            .filter(|eq| self.can_solve_equation(eq))
            .map(|eq| eq.result)
            .sum()
        )
    }
}

impl Day {
    fn can_solve_equation(&self, eq: &Equation) -> bool {
        let result = eq.parts[0];
        self.try_operations(&eq.parts[1..], result, eq.result)
    }

    fn try_operations(&self, parts: &[u64], current: u64, target: u64) -> bool {
        if parts.is_empty() {
            return current == target;
        }

        for operation in &self.operations {
            if self.try_operations(&parts[1..], operation(current, parts[0]), target) {
                return true;
            }
        }
        false
    }
}