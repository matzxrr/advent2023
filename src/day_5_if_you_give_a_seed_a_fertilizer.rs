use std::{
    sync::{Arc, Mutex},
    thread,
};

const INPUT: &str = include_str!("assets/day_5_input_1.txt");

pub fn exec_star_9() -> i32 {
    star_9(INPUT)
}

trait Header {
    const HEADER: &'static str;
}

#[derive(Debug, Clone)]
struct Mapping {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

impl Mapping {
    fn new(start: u64, source: u64, range: u64) -> Self {
        Self {
            destination_start: start,
            source_start: source,
            range_length: range,
        }
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fert: Vec<Mapping>,
    fert_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temp: Vec<Mapping>,
    temp_to_humid: Vec<Mapping>,
    humid_to_loc: Vec<Mapping>,
}

impl Almanac {
    fn new() -> Self {
        Self {
            seeds: vec![],
            seed_to_soil: vec![],
            soil_to_fert: vec![],
            fert_to_water: vec![],
            water_to_light: vec![],
            light_to_temp: vec![],
            temp_to_humid: vec![],
            humid_to_loc: vec![],
        }
    }
    fn parse_group(&mut self, group: &[&str]) {
        if group[0].starts_with("seeds:") {
            self.parse_seeds(group);
        }
        if group[0].starts_with("seed-to-soil") {
            self.seed_to_soil = Almanac::parse_mapping(group);
        }
        if group[0].starts_with("soil-to-fertilizer") {
            self.soil_to_fert = Almanac::parse_mapping(group);
        }
        if group[0].starts_with("fertilizer-to-water") {
            self.fert_to_water = Almanac::parse_mapping(group);
        }
        if group[0].starts_with("water-to-light") {
            self.water_to_light = Almanac::parse_mapping(group);
        }
        if group[0].starts_with("light-to-temperature") {
            self.light_to_temp = Almanac::parse_mapping(group);
        }
        if group[0].starts_with("temperature-to-humidity") {
            self.temp_to_humid = Almanac::parse_mapping(group);
        }
        if group[0].starts_with("humidity-to-location") {
            self.humid_to_loc = Almanac::parse_mapping(group);
        }
    }
    fn parse_seeds(&mut self, input: &[&str]) {
        self.seeds = input[0]
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
    }
    fn parse_mapping(input: &[&str]) -> Vec<Mapping> {
        let mut lines = input.iter();
        lines.next(); // Skip first line
        lines
            .map(|l| {
                let s: Vec<_> = l
                    .split(' ')
                    .map(|n| n.trim())
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect();
                s
            })
            .map(|n| Mapping::new(n[0], n[1], n[2]))
            .collect::<Vec<_>>()
    }
    fn find_next(mappings: &Vec<Mapping>, start: u64) -> u64 {
        for mapping in mappings {
            if start >= mapping.source_start && start <= mapping.source_start + mapping.range_length
            {
                let offset = start - mapping.source_start;
                return mapping.destination_start + offset;
            }
        }
        start
    }
}

fn star_9(input: &str) -> i32 {
    let lines = input.lines();
    let groups = lines.fold(vec![vec![]], |mut acc, value| {
        if value.is_empty() {
            acc.push(vec![]);
        } else {
            let index = acc.len() - 1;
            acc[index].push(value);
        }
        acc
    });

    let mut almanac = Almanac::new();
    for group in groups.iter() {
        almanac.parse_group(group);
    }

    // seed and location
    let mut closets_seed = (u64::MAX, u64::MAX);
    for seed in almanac.seeds {
        let soil = Almanac::find_next(&almanac.seed_to_soil, seed);
        let fert = Almanac::find_next(&almanac.soil_to_fert, soil);
        let water = Almanac::find_next(&almanac.fert_to_water, fert);
        let light = Almanac::find_next(&almanac.water_to_light, water);
        let temp = Almanac::find_next(&almanac.light_to_temp, light);
        let humid = Almanac::find_next(&almanac.temp_to_humid, temp);
        let loc = Almanac::find_next(&almanac.humid_to_loc, humid);
        if loc < closets_seed.1 {
            closets_seed = (seed, loc);
        }
    }
    closets_seed.1 as i32
}

pub fn exec_star_10() -> i32 {
    star_10(INPUT)
}

fn star_10(input: &str) -> i32 {
    let lines = input.lines();
    let groups = lines.fold(vec![vec![]], |mut acc, value| {
        if value.is_empty() {
            acc.push(vec![]);
        } else {
            let index = acc.len() - 1;
            acc[index].push(value);
        }
        acc
    });

    let mut almanac = Almanac::new();
    for group in groups.iter() {
        almanac.parse_group(group);
    }

    let closet_seed = Arc::new(Mutex::new((u64::MAX, u64::MAX)));
    let mut handles = vec![];

    let len = almanac.seeds.len();
    let mut index = 0;
    while index < len {
        let seed = almanac.seeds[index];
        let range = almanac.seeds[index + 1];
        let almanac_clone = almanac.clone();
        let closet = Arc::clone(&closet_seed);
        index += 2;
        let handle = thread::spawn(move || {
            let mut close = (u64::MAX, u64::MAX);
            for start_seed in seed..=seed + range {
                let soil = Almanac::find_next(&almanac_clone.seed_to_soil, start_seed);
                let fert = Almanac::find_next(&almanac_clone.soil_to_fert, soil);
                let water = Almanac::find_next(&almanac_clone.fert_to_water, fert);
                let light = Almanac::find_next(&almanac_clone.water_to_light, water);
                let temp = Almanac::find_next(&almanac_clone.light_to_temp, light);
                let humid = Almanac::find_next(&almanac_clone.temp_to_humid, temp);
                let loc = Almanac::find_next(&almanac_clone.humid_to_loc, humid);
                if loc < close.1 {
                    close = (start_seed, loc);
                }
            }
            let mut c = closet.lock().unwrap();
            if close.1 < c.1 {
                *c = (close.0, close.1);
            }
        });
        println!("Thread spawned : #{}", handles.len() + 1);
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let x = closet_seed.lock().unwrap().1 as i32;
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_9() {
        let input = include_str!("assets/day_5_test_input_1.txt");
        let result = star_9(input);
        assert_eq!(result, 35);
    }
    #[test]
    fn test_start_10() {
        let input = include_str!("assets/day_5_test_input_1.txt");
        let result = star_10(input);
        assert_eq!(result, 46);
    }
}
