use std::ops::Range;
use std::str::FromStr;

use crate::aoc::Error;

#[derive(Default, Clone, Debug)]
pub struct Relation {
    destination_range: Range<u64>,
    source_range: Range<u64>,
}

#[derive(Clone, Debug, Default)]
struct FromToRelations {
    relations: Vec<Relation>,
}

impl FromToRelations {
    fn mapping(&self, target: u64) -> u64 {
        for relation in &self.relations {
            let mut source = relation.source_range.clone();
            source.end += 1;

            let mut dest = relation.destination_range.clone();
            dest.end += 1;

            if target >= source.start && target < source.end {
                let index = target - source.start;
                let value = dest.start + index;
                if value < dest.end {
                    return value;
                }
            }
        }

        target
    }
}

#[derive(Debug, Default, Clone)]
pub struct Day {
    seeds: Vec<u64>,
    from_to_relations: Vec<FromToRelations>,
}

impl crate::aoc::Day for Day {
    type Output = u64;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![("seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4", 35)]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![("seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4", 46)]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut min = u64::MAX;

        for seed in &self.seeds {
            let seed = *seed;
            let final_seed = from_location_to_location(&self.from_to_relations, seed, 0, self.from_to_relations.len());

            if final_seed < min {
                min = final_seed;
            }
        }

        Ok(min)
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let mut seeds = self.seeds.chunks(2).map(|chunk| {
            chunk[0]..(chunk[0] + chunk[1])
        }).collect::<Vec<_>>();

        // overall algorithm. for each map seed-to-soil, soil-to-fertilizer etc.
        // check if there is an overlap with one of the seed ranges.
        // if there is an overlap check for cases, in where the overlap is completly in the map area
        // if so, look at this inner section again
        // if not the outer part considered mapped
        for from_to_relation in &self.from_to_relations {
            let mut new = vec![];

            while let Some(current_seed_range) = seeds.pop() {
                let mut broken = false;

                // check the overlap for each relation
                for relation in &from_to_relation.relations {
                    let destination_start = relation.destination_range.start;

                    let source_start = relation.source_range.start;
                    let source_end = relation.source_range.end;

                    // calculate the overlaps
                    // overlap start and overlap end
                    let os = current_seed_range.start.max(source_start);
                    let oe = current_seed_range.end.min(source_end);

                    if os < oe { // if it overlaps
                        new.push(os - source_start + destination_start..oe - source_start + destination_start);


                        // handle cases, if the overlap is completely in the map-range
                        if os > current_seed_range.start {
                            seeds.push(current_seed_range.start..os)
                        }

                        if current_seed_range.end > oe {
                            seeds.push(oe..current_seed_range.end);
                        }

                        broken = true;
                        break;
                    }
                }

                if !broken {
                    new.push(current_seed_range)
                }
            }

            seeds = new;
        }

        seeds.sort_by(|a, b| a.start.cmp(&b.start));
        Ok(seeds[0].start)
    }
}

fn from_location_to_location(from_to_relations: &[FromToRelations], seed: u64, level: usize, max_level: usize) -> u64 {
    if level == max_level {
        return seed;
    }

    let new_seed = from_to_relations[level].mapping(seed);
    from_location_to_location(from_to_relations, new_seed, level + 1, max_level)
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let seeds: Vec<u64> = if let Some(line) = lines.next() {
            if let ["seeds:", numbers @ ..] = &line.split(' ').collect::<Vec<_>>()[..] {
                numbers.iter()
                    .map(|number_str| number_str.parse::<u64>().unwrap_or(0))
                    .collect::<Vec<_>>()
            } else {
                return Err(Error::StringParse(line.to_string()));
            }
        } else {
            return Err(Error::StringParse(String::new()));
        };

        let mut relations = vec![];
        let mut current_relation: FromToRelations = FromToRelations { relations: vec![] };

        for line in lines {
            if line.is_empty() {
                if !current_relation.relations.is_empty() { // new relation
                    relations.push(current_relation);
                    current_relation = FromToRelations { relations: vec![] };
                }
                continue;
            }

            if line.contains(':') {
                continue;
            }

            if let [destination_range_start, source_range_start, range_length] = line.split(' ').collect::<Vec<_>>()[..] {
                let destination_range_start = destination_range_start.parse::<u64>()?;
                let source_range_start = source_range_start.parse::<u64>()?;
                let range_length = range_length.parse::<u64>()?;

                let relation = Relation {
                    destination_range: destination_range_start..(destination_range_start + range_length) - 1,
                    source_range: source_range_start..(source_range_start + range_length) - 1,
                };

                current_relation.relations.push(relation);
            }
        }

        if !current_relation.relations.is_empty() { // new relation
            relations.push(current_relation);
        }

        Ok(Self {
            seeds,
            from_to_relations: relations,
        })
    }
}