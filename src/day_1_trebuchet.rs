pub mod day_1 {
    pub fn star_1(input: &str) -> i32 {
        println!("{}", input);
        0
    }
}

#[cfg(test)]
mod day_1_test {
    use super::*;

    #[test]
    fn test_star_1() {
        let input = include_str!("assets/star_1_test.txt");
        let result = day_1::star_1(input);
        assert_eq!(result, 77);
    }
}
