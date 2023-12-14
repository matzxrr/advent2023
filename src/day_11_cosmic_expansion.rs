const INPUT: &str = include_str!("assets/day_11_input_1.txt");

pub fn exec_star_21() -> i32 {
    star_21(INPUT)
}

fn star_21(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_21() {
        let input = include_str!("assets/day_11_test_input_1.txt");
        let result = star_21(input);
        assert_eq!(result, 374);
    }
}
