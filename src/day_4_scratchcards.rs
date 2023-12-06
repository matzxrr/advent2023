use std::collections::HashMap;

pub fn exec_star_7() -> i32 {
    let input = include_str!("assets/day_4_input_1.txt");
    star_7(input)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_7() {
        let input = include_str!("assets/day_4_test_input_1.txt");
        let result = star_7(input);
        assert_eq!(result, 13);
    }
}
