const INPUT: &str = include_str!("assets/day_9_input_1.txt");

#[derive(Debug)]
struct Sensor {
    numbers: Vec<Vec<Vec<i64>>>,
}

impl From<&str> for Sensor {
    fn from(input: &str) -> Self {
        let mut numbers = Vec::new();
        for line in input.lines() {
            let set = line
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>();
            numbers.push(vec![set]);
        }
        Self { numbers }
    }
}

pub fn exec_star_17() -> i64 {
    star_17(INPUT)
}

fn star_17(input: &str) -> i64 {
    let mut sensor = Sensor::from(input);
    let mut total = 0;
    for set in sensor.numbers.iter_mut() {
        let mut row_index = 0;
        loop {
            let looking_row = &set[row_index];
            let len = looking_row.len();
            let mut new_row = vec![];
            for counter in 0..len - 1 {
                let a = counter;
                let b = a + 1;
                new_row.push(looking_row[b] - looking_row[a]);
            }
            if new_row.iter().all(|n| n == &0) {
                set.push(new_row);
                break;
            }
            set.push(new_row);
            row_index += 1;
        }
        let mut new_value = 0;
        let mut iter = set.iter();
        while let Some(row) = iter.next_back() {
            new_value += row[row.len() - 1];
        }
        total += new_value;
    }
    total
}

pub fn exec_star_18() -> i64 {
    star_18(INPUT)
}

fn star_18(input: &str) -> i64 {
    let mut sensor = Sensor::from(input);
    let mut total = 0;
    for set in sensor.numbers.iter_mut() {
        let mut row_index = 0;
        loop {
            let looking_row = &set[row_index];
            let len = looking_row.len();
            let mut new_row = vec![];
            for counter in 0..len - 1 {
                let a = counter;
                let b = a + 1;
                new_row.push(looking_row[b] - looking_row[a]);
            }
            if new_row.iter().all(|n| n == &0) {
                set.push(new_row);
                break;
            }
            set.push(new_row);
            row_index += 1;
        }
        let mut new_value = 0;
        let mut iter = set.iter();
        while let Some(row) = iter.next_back() {
            new_value = row[0] - new_value;
        }
        total += new_value;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_17() {
        let input = include_str!("assets/day_9_test_input_1.txt");
        let result = star_17(input);
        assert_eq!(result, 114);
    }
    #[test]
    fn test_star_18() {
        let input = include_str!("assets/day_9_test_input_1.txt");
        let result = star_18(input);
        assert_eq!(result, 2);
    }
}
