pub fn exec_star_1() -> i32 {
    let input = include_str!("assets/star_1_input.txt");
    star_1(input)
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
        println!("{}", result);
        total += result.parse::<i32>().unwrap();
    }
    total
}

#[cfg(test)]
mod day_1_test {
    use super::*;

    #[test]
    fn test_star_1() {
        let input = include_str!("assets/star_1_test.txt");
        let result = star_1(input);
        assert_eq!(result, 142);
    }
}
