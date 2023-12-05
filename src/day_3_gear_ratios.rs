const INPUT: &str = include_str!("assets/day_3_input_1.txt");

pub fn exec_star_5() -> i32 {
    star_5(INPUT)
}

fn star_5(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let mut valid_schematics: Vec<String> = vec![];
    for i in 0..lines.len() {
        let prev_line = if i == 0 { None } else { lines.get(i - 1) };
        let current_line = lines[i];
        let next_line = lines.get(i + 1);

        let chars: Vec<_> = current_line.chars().collect();
        let mut j = 0;
        'charloop: while j < chars.len() {
            if chars[j].is_ascii_digit() {
                let j_start = j;
                let mut number = String::from(chars[j]);
                while chars.get(j + 1).is_some_and(|n| n.is_ascii_digit()) {
                    j += 1;
                    number.push(chars[j]);
                }

                // Number we found
                dbg!(&number);

                // Data about the number
                let number_len = number.len();
                let range = number_len + 2; // left and right

                // Check previous line
                if let Some(above) = prev_line {
                    let chars: Vec<_> = above.chars().collect();
                    let start = if j_start == 0 { 0 } else { j_start - 1 };
                    let end = if start + range > above.len() {
                        above.len() - 1
                    } else {
                        start + range
                    };
                    for a in chars.iter().take(end).skip(start) {
                        if !a.is_ascii_digit() && *a != '.' {
                            valid_schematics.push(number);
                            j += 1;
                            continue 'charloop;
                        }
                    }
                }

                // Check current line
                let left = if j_start == 0 { 0 } else { j_start - 1 };
                let right = if left + range > current_line.len() {
                    current_line.len() - 1
                } else {
                    j_start + number_len
                };
                if chars[left] != '.' && !chars[left].is_ascii_digit()
                    || chars[right] != '.' && !chars[right].is_ascii_digit()
                {
                    valid_schematics.push(number);
                    j += 1;
                    continue 'charloop;
                }

                // Check next line
                if let Some(next) = next_line {
                    let chars: Vec<_> = next.chars().collect();
                    let start = if j_start == 0 { 0 } else { j_start - 1 };
                    let end = if start + range > next.len() {
                        next.len() - 1
                    } else {
                        start + range
                    };
                    for a in chars.iter().take(end).skip(start) {
                        if !a.is_ascii_digit() && *a != '.' {
                            valid_schematics.push(number);
                            j += 1;
                            continue 'charloop;
                        }
                    }
                }
            }
            j += 1;
        }
    }

    valid_schematics
        .iter()
        .fold(0, |acc, v| acc + v.parse::<i32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_5() {
        let input = include_str!("assets/day_3_test_input_1.txt");
        let result = star_5(input);
        assert_eq!(result, 4361);
    }
}
