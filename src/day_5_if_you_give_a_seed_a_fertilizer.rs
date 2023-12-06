#![allow(dead_code)]
#![allow(unused_variables)]

const INPUT: &str = include_str!("assets/day_5_input_1.txt");

pub fn exec_star_9() -> i32 {
    star_9(INPUT)
}

trait Header {
    const HEADER: &'static str;
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i32>,
}

impl Almanac {
    fn new() -> Self {
        Self { seeds: vec![] }
    }
    fn parse_group(&mut self, group: &[&str]) {
        if group[0].starts_with("seeds:") {
            self.parse_seeds(group);
        }
    }
    fn parse_seeds(&mut self, input: &[&str]) {
        self.seeds = input[0]
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
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

    dbg!(&almanac);
    0
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
}
