pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;


pub trait Day {
    fn date(&self) -> (i32, i32);
    fn run(&self);
}

#[derive(Default)]
pub struct AdventRunner {
    days: Vec<Box<dyn Day>>,
    current_year: i32,
}

impl AdventRunner {
    pub fn new() -> Self {
        let days: Vec<Box<dyn Day>> = Vec::new();

        AdventRunner {
            current_year: 2022,
            days
        }
    }

    pub fn set_year(mut self, year: i32) -> Self {
        self.current_year = year;
        self
    }


    pub fn run(self) {
        let latest_year = 2022;
        if self.current_year == latest_year {
            let day = self.days.last().unwrap().clone();
            println!("Day: {}, Year {}", day.date().0, day.date().1);
            day.run();
            return;
        }

        for (i, day) in self.days.iter().enumerate() {
            if day.date().1 > self.current_year {
                if let Some(day) = self.days.get(i - 1) {
                    println!("Day: {}, Year {}", day.date().0, day.date().1);
                    day.run();
                }

                return;
            }
        }

        // at this point, just run the latest
        let day = self.days.last().unwrap();
        println!("Day: {}, Year {}", day.date().0, day.date().1);

        day.run();
    }
}