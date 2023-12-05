const INPUT: &str = include_str!("assets/day_3_input_1.txt");

pub fn exec_star_5() -> i32 {
    star_5(INPUT)
}

fn star_5(input: &str) -> i32 {
    todo!()
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
