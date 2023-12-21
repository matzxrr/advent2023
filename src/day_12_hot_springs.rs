const INPUT: &str = include_str!("assets/day_12_input_1.txt");

#[derive(Debug)]
struct Springs<'a> {
    map: Vec<(&'a str, Vec<i32>)>,
}

impl<'a> From<&'a str> for Springs<'a> {
    fn from(value: &'a str) -> Self {
        let map = value
            .lines()
            .map(|l| {
                let (left, right) = l.split_once(' ').unwrap();
                (
                    left,
                    right
                        .split(',')
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect(),
                )
            })
            .collect();
        Self { map }
    }
}

pub fn exec_star_24() -> i32 {
    star_24(INPUT)
}

fn star_24(input: &str) -> i32 {
    0
}

pub fn exec_star_23() -> i32 {
    star_23(INPUT)
}

fn star_23(input: &str) -> i32 {
    let springs = Springs::from(input);
    dbg!(&springs);
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
