use std::collections::HashMap;

const INPUT: &str = include_str!("assets/day_4_input_1.txt");

pub fn exec_star_7() -> i32 {
    star_7(INPUT)
}

#[derive(Debug)]
struct Scratcher {
    instances: usize,
    matches: usize,
}

impl Scratcher {
    fn new(raw_string: &str) -> Self {
        let (_, game_numbers) = raw_string.split_once(':').unwrap();
        // let (_, game_id) = game.trim().split_once(' ').unwrap();
        let (winning_numbers, numbers_you_have) = game_numbers.trim().split_once('|').unwrap();
        let mut matches = 0;
        let winning: HashMap<i32, _> = winning_numbers
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| (n.parse::<i32>().unwrap(), ()))
            .collect();
        let scratched: Vec<_> = numbers_you_have
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        for number in &scratched {
            if winning.get(number).is_some() {
                matches += 1;
            }
        }
        Scratcher {
            instances: 1,
            matches,
        }
    }
}

fn star_7(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let (_, game_numbers) = line.split_once(':').unwrap();
        let (winning_numbers, numbers_you_have) = game_numbers.trim().split_once('|').unwrap();
        let mut wins = 0;
        let winning: HashMap<i32, _> = winning_numbers
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| (n.parse::<i32>().unwrap(), ()))
            .collect();
        let scratched: Vec<_> = numbers_you_have
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        for ref number in scratched {
            if winning.get(number).is_some() {
                if wins == 0 {
                    wins = 1;
                } else {
                    wins *= 2;
                }
            }
        }
        total += wins;
    }
    total
}

pub fn exec_star_8() -> i32 {
    star_8(INPUT)
}

fn star_8(input: &str) -> i32 {
    let mut scratchers: HashMap<usize, Scratcher> = HashMap::new();
    for (index, line) in input.lines().enumerate() {
        scratchers.insert(index, Scratcher::new(line));
    }
    let line_count = input.lines().count();
    for index in 0..line_count {
        if let Some(scratcher) = scratchers.get(&index) {
            let matches = scratcher.matches;
            let instances = scratcher.instances;
            let start = if index + 1 < line_count {
                index + 1
            } else {
                line_count
            };
            let end = if start + matches < line_count {
                start + matches
            } else {
                line_count
            };
            for j in start..end {
                scratchers.entry(j).and_modify(|x| {
                    x.instances += instances;
                });
            }
        }
    }
    scratchers.values().fold(0, |mut acc, v| {
        acc += v.instances as i32;
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_7() {
        let input = include_str!("assets/day_4_test_input_1.txt");
        let result = star_7(input);
        assert_eq!(result, 13);
    }
    #[test]
    fn test_star_8() {
        let input = include_str!("assets/day_4_test_input_1.txt");
        let result = star_8(input);
        assert_eq!(result, 30);
    }
}
