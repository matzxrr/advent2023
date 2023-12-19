const INPUT: &str = include_str!("assets/day_12_input_1.txt");

pub fn exec_star_23() -> i32 {
    star_23(INPUT)
}

fn star_23(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_23() {
        let input = include_str!("assets/day_12_test_input_1.txt");
        let result = star_23(input);
        assert_eq!(result, 21);
    }
}
