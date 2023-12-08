const INPUT: &str = include_str!("assets/day_6_input_1.txt");

pub fn exec_star_11() -> i32 {
    star_11(INPUT)
}

fn star_11(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_11() {
        let input = include_str!("assets/day_6_test_input_1.txt");
        let result = star_11(input);
        assert_eq!(result, 288);
    }
}
