const INPUT: &str = include_str!("assets/day_10_input_1.txt");

#[derive(Debug)]
struct PipeMaze {
    rows: Vec<Vec<char>>,
    starting_position: (usize, usize),
}

impl From<&str> for PipeMaze {
    fn from(value: &str) -> Self {
        let mut starting_position = (0, 0);
        let mut rows = Vec::new();
        for (index, line) in value.lines().enumerate() {
            let chars = line.chars();
            rows.push(chars.clone().collect());
            if let Some(pos) = chars.enumerate().find(|c| c.1 == 'S') {
                starting_position = (index, pos.0);
            }
        }
        Self {
            rows,
            starting_position,
        }
    }
}
pub fn exec_star_20() -> i64 {
    unimplemented!()
}

pub fn exec_star_19() -> i64 {
    star_19(INPUT)
}

fn star_19(input: &str) -> i64 {
    let maze = PipeMaze::from(input);
    dbg!(&maze);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_19_1() {
        let input = include_str!("assets/day_10_test_input_1.txt");
        let result = star_19(input);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_star_19_2() {
        let input = include_str!("assets/day_10_test_input_2.txt");
        let result = star_19(input);
        assert_eq!(result, 8);
    }
}
