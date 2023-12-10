#![allow(dead_code)]
use std::collections::HashMap;

const INPUT: &str = include_str!("assets/day_7_input_1.txt");

#[derive(Debug)]
struct Hand<'a> {
    card_string: &'a str,
    bid: i32,
    hand_value: i32,
    cards_as_values: Vec<i32>,
}
impl<'a> From<(&'a str, &'a str)> for Hand<'a> {
    fn from(value: (&'a str, &'a str)) -> Hand {
        Hand {
            card_string: value.0,
            bid: value.1.parse().unwrap(),
            hand_value: get_hand_value(value.0),
            cards_as_values: value.0.chars().map(|c| get_char_value(&c)).collect(),
        }
    }
}

/// high card: 1
/// one pair : 2
/// two pair : 3
/// threeofk : 4
/// fullhous : 5
/// fourofk  : 6
/// fiveofk  : 7

fn get_hand_value(card_string: &str) -> i32 {
    let mut hm: HashMap<char, i32> = HashMap::new();
    card_string.chars().for_each(|c| {
        hm.entry(c).and_modify(|n| *n += 1).or_insert(1);
    });
    let mut sorted = hm.values().collect::<Vec<_>>();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
    match sorted[0] {
        1 => 1, // High Card
        2 => {
            if *sorted[1] == 2 {
                3 // two pair
            } else {
                2 // one pair
            }
        }
        3 => {
            if *sorted[1] == 2 {
                5 // full house
            } else {
                4 // three of a kind
            }
        }
        4 => 6, // four of a kind
        5 => 7, // five of a kind
        _ => unreachable!(),
    }
}

/// 2    : 1
/// 3    : 2
/// 4    : 3
/// 5    : 4
/// 6    : 5
/// 7    : 6
/// 8    : 7
/// 9    : 8
/// T    : 9
/// J    : 10
/// Q    : 11
/// K    : 12
/// A    : 13

fn get_char_value(ch: &char) -> i32 {
    match ch {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => unreachable!(),
    }
}

pub fn exec_star_13() -> i32 {
    star_13(INPUT)
}

fn star_13(input: &str) -> i32 {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| Hand::from(l.split_once(' ').unwrap()))
        .collect();
    hands.sort_by(|a, b| {
        if a.hand_value != b.hand_value {
            a.hand_value.partial_cmp(&b.hand_value).unwrap()
        } else {
            let mut index = 0;
            while a.cards_as_values[index] == b.cards_as_values[index] {
                index += 1;
            }
            a.cards_as_values[index]
                .partial_cmp(&b.cards_as_values[index])
                .unwrap()
        }
    });
    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (index, hand)| acc + hand.bid * (index as i32 + 1));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_13() {
        let input = include_str!("assets/day_7_test_input_1.txt");
        let result = star_13(input);
        assert_eq!(result, 6440);
    }
}
