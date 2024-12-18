use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

pub trait Day: Default + FromStr + Clone + Sized {
    type Output: Display + Debug;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)>;
    fn test_cases_2() -> Vec<(&'static str, Self::Output)>;

    fn after_test_1(&mut self) { }
    fn after_test_2(&mut self) { }

    fn test_1(&self) -> anyhow::Result<()> where <Self as FromStr>::Err: std::error::Error, <Self as FromStr>::Err: Send, <Self as FromStr>::Err: Sync {
        for (test_case, result) in Self::test_cases_1() {
            let day = Self::from_str(test_case);

            if let Ok(mut day) = day {
                assert_eq!(format!("{}", day.solution1()?), format!("{}", result))
            }
        }

        Ok(())
    }

    fn test_2(&self) -> anyhow::Result<()> where <Self as FromStr>::Err: std::error::Error, <Self as FromStr>::Err: Send, <Self as FromStr>::Err: Sync {
        for (test_case, result) in Self::test_cases_2() {
            let day = Self::from_str(test_case);

            if let Ok(mut day) = day {
                assert_eq!(format!("{}", day.solution2()?), format!("{}", result))
            }
        }

        Ok(())
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output>;
    fn solution2(&mut self) -> anyhow::Result<Self::Output>;
}



#[derive(Debug)]
pub enum Error {
    StringParse(String),
    GridError(crate::utils::grid::Error),
    NoSolutionFound
}

impl From<crate::utils::grid::Error> for Error {
    fn from(value: crate::utils::grid::Error) -> Self {
        Error::GridError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::StringParse(value.to_string())
    }
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Error::StringParse(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::StringParse(message) => format!("Cannot parse message: \"{message}\""),
            Error::NoSolutionFound => "No solution has been found".to_string(),
            Error::GridError(v) => format!("{}", v)
        })
    }
}

impl std::error::Error for Error { }