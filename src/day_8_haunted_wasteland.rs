use std::collections::HashMap;

const INPUT: &str = include_str!("assets/day_8_input_1.txt");

#[derive(Debug)]
struct Map {
    directions: Vec<char>,
    map: HashMap<String, Vec<String>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let chars: Vec<_> = lines.next().unwrap().chars().collect();
        lines.next(); // skip empty line
        let map = lines
            .map(|l| {
                let (left, right) = l.split_once('=').unwrap();
                let between = right.trim().replace(&['(', ')'][..], "");
                let parts = between.split_once(',').unwrap();
                (
                    left.trim().to_owned(),
                    vec![parts.0.trim().to_owned(), parts.1.trim().to_owned()],
                )
            })
            .collect();
        Self {
            directions: chars,
            map,
        }
    }
}

pub fn exec_star_15() -> i32 {
    star_15(INPUT)
}

fn star_15(input: &str) -> i32 {
    let mut steps_taken = 0;
    let map = Map::from(input);
    let mut current_location = String::from("AAA");
    let end_location = String::from("ZZZ");
    let mut direction_index = 0;
    while current_location != end_location {
        let direction = if map.directions[direction_index] == 'L' {
            0
        } else {
            1
        };
        let next = map.map.get(&current_location).unwrap();
        current_location = next[direction].clone();

        steps_taken += 1;
        if direction_index < map.directions.len() - 1 {
            direction_index += 1;
        } else {
            direction_index = 0;
        }
    }
    steps_taken
}

pub fn exec_star_16() -> u64 {
    star_16(INPUT)
}

fn star_16(input: &str) -> u64 {
    let map = Map::from(input);
    let current_locations = map.map.keys().filter(|s| s.ends_with('A'));
    let mut results = Vec::new();
    for location in current_locations {
        let mut direction_index = 0;
        let mut step_count: u64 = 0;
        let mut cl = location;
        while !cl.ends_with('Z') {
            let direction = if map.directions[direction_index] == 'L' {
                0
            } else {
                1
            };
            let next = map.map.get(cl).unwrap();
            cl = &next[direction];
            step_count += 1;
            if direction_index < map.directions.len() - 1 {
                direction_index += 1;
            } else {
                direction_index = 0;
            }
        }
        results.push(step_count);
    }
    results.into_iter().reduce(lcm).unwrap()
}

fn lcm(a: u64, b: u64) -> u64 {
    let (mut x, mut y) = if a > b { (a, b) } else { (b, a) };
    let mut rem = x % y;
    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }
    a * b / y
}

#[cfg(test)]
mod test_star_15 {
    use super::*;

    #[test]
    fn test_star_15_1() {
        let input = include_str!("assets/day_8_test_input_1.txt");
        let result = star_15(input);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_star_15_2() {
        let input = include_str!("assets/day_8_test_input_2.txt");
        let result = star_15(input);
        assert_eq!(result, 6);
    }
    #[test]
    fn test_star_16_1() {
        let input = include_str!("assets/day_8_test_input_3.txt");
        let result = star_16(input);
        assert_eq!(result, 6);
    }
}
