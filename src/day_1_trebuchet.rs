use std::collections::HashMap;

const INPUT_TO_PARSE: &str = include_str!("assets/day_1_input_1.txt");

pub fn exec_star_1() -> i32 {
    star_1(INPUT_TO_PARSE)
}

fn star_1(input: &str) -> i32 {
    let lines = input.lines();
    let mut total = 0;
    for line in lines {
        let mut result = String::new();
        for char in line.chars() {
            if char.is_numeric() {
                result.push(char);
                break;
            }
        }
        let mut iter = line.chars();
        while let Some(char) = iter.next_back() {
            if char.is_numeric() {
                result.push(char);
                break;
            }
        }
        total += result.parse::<i32>().unwrap();
    }
    total
}

pub fn exec_star_2() -> i32 {
    star_2(INPUT_TO_PARSE)
}

fn star_2(input: &str) -> i32 {
    let number_words_two = HashMap::from([
        ('o', vec![("one", '1')]),
        ('t', vec![("two", '2'), ("three", '3')]),
        ('f', vec![("four", '4'), ("five", '5')]),
        ('s', vec![("six", '6'), ("seven", '7')]),
        ('e', vec![("eight", '8')]),
        ('n', vec![("nine", '9')]),
    ]);
    let mut total = 0;
    for line in input.lines() {
        let mut numbers_as_char_in_string: Vec<char> = vec![];
        'sub: for (index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                numbers_as_char_in_string.push(char);
                continue 'sub;
            }
            if let Some(words) = number_words_two.get(&char) {
                'word: for (word, v) in words {
                    if index + word.len() <= line.len() {
                        let line_word = &line[index..index + word.len()];
                        if *word == line_word {
                            numbers_as_char_in_string.push(*v);
                            break 'word;
                        }
                    }
                }
            }
        }
        let mut s = String::from(numbers_as_char_in_string[0]);
        s.push(numbers_as_char_in_string[numbers_as_char_in_string.len() - 1]);
        total += s.parse::<i32>().unwrap();
    }
    total
}

#[cfg(test)]
mod day_1_test {
    use super::*;

    #[test]
    fn test_star_1() {
        let input = include_str!("assets/day_1_test_input_1.txt");
        let result = star_1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_star_2() {
        let input = include_str!("assets/day_1_test_input_2.txt");
        let result = star_2(input);
        assert_eq!(result, 281);
    }
}
