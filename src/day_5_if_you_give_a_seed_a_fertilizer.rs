const INPUT: &str = include_str!("assets/day_5_input_1.txt");

pub fn exec_star_9() -> i32 {
    star_9(INPUT)
}

fn star_9(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_9() {
        let input = include_str!("assets/day_5_test_input_1.txt");
        let result = star_9(input);
        assert_eq!(result, 35);
    }
}
