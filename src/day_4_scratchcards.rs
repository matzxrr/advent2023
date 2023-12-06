pub fn exec_star_7() -> i32 {
    let input = include_str!("assets/day_4_input_1.txt");
    star_7(input)
}

fn star_7(input: &str) -> i32 {
    todo!()
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
