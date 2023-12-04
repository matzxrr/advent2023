const INPUT_TO_PARSE: &str = include_str!("assets/star_3_input.txt");

pub fn exec_star_3() -> i32 {
    star_3(INPUT_TO_PARSE)
}

fn star_3(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_3() {
        let input = include_str!("assets/star_3_test.txt");
        let result = star_3(input);
        assert_eq!(result, 8);
    }
}
