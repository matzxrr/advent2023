const INPUT: &str = include_str!("assets/day_11_input_1.txt");

#[derive(Debug)]
struct Galaxy {
    map: Vec<Vec<char>>,
}

impl From<&str> for Galaxy {
    fn from(value: &str) -> Self {
        let map = value.lines().map(|l| l.chars().collect()).collect();
        Self { map }
    }
}

impl Galaxy {
    #[allow(dead_code)]
    fn draw(&self) {
        for row in self.map.iter() {
            let mut s = String::from(row[0]);
            (1..row.len()).for_each(|i| {
                s.push(row[i]);
            });
            println!("{}", s);
        }
    }
    fn expand(&mut self) {
        let mut x_map = vec![false; self.map[0].len()];
        let mut y_map = vec![false; self.map.len()];

        for (ref y_index, y_row) in self.map.iter().enumerate() {
            let mut row_has = false;
            for (ref x_index, x_item) in y_row.iter().enumerate() {
                if x_item == &'#' {
                    x_map[*x_index] = true;
                    row_has = true;
                }
            }
            if row_has {
                y_map[*y_index] = true;
            }
        }

        let x_voided: Vec<_> = x_map
            .iter()
            .enumerate()
            .filter(|n| !*n.1)
            .map(|l| l.0)
            .collect();

        for x in x_voided.iter().enumerate() {
            for row in self.map.iter_mut() {
                row.insert(x.1 + x.0, '.');
            }
        }

        let y_voided: Vec<_> = y_map
            .iter()
            .enumerate()
            .filter(|n| !*n.1)
            .map(|l| l.0)
            .collect();

        for y in y_voided.iter().enumerate() {
            self.map.insert(y.1 + y.0, vec!['.'; self.map[0].len()]);
        }
    }
    fn get_expand_map(&self) -> (Vec<usize>, Vec<usize>) {
        let mut x_map = vec![false; self.map[0].len()];
        let mut y_map = vec![false; self.map.len()];

        for (ref y_index, y_row) in self.map.iter().enumerate() {
            let mut row_has = false;
            for (ref x_index, x_item) in y_row.iter().enumerate() {
                if x_item == &'#' {
                    x_map[*x_index] = true;
                    row_has = true;
                }
            }
            if row_has {
                y_map[*y_index] = true;
            }
        }
        let x_voided: Vec<_> = x_map
            .iter()
            .enumerate()
            .filter(|n| !*n.1)
            .map(|l| l.0)
            .collect();
        let y_voided: Vec<_> = y_map
            .iter()
            .enumerate()
            .filter(|n| !*n.1)
            .map(|l| l.0)
            .collect();
        (x_voided, y_voided)
    }
    fn get_locations(&self) -> Vec<(i32, i32)> {
        let mut locations = vec![];
        for (y, row) in self.map.iter().enumerate() {
            for (x, char) in row.iter().enumerate() {
                if char == &'#' {
                    locations.push((x as i32, y as i32));
                }
            }
        }
        locations
    }
}

pub fn exec_star_21() -> i32 {
    star_21(INPUT)
}

fn star_21(input: &str) -> i32 {
    let mut map = Galaxy::from(input);
    map.expand();
    let locations = map.get_locations();
    let mut total = 0;
    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            let a = locations[i];
            let b = locations[j];
            let distance = (a.0 - b.0).abs() + (a.1 - b.1).abs();
            total += distance;
        }
    }
    total
}

pub fn exec_star_22() -> i32 {
    star_22(INPUT, 1000000)
}

fn star_22(input: &str, expand: i32) -> i32 {
    let map = Galaxy::from(input);
    let (xs, ys) = map.get_expand_map();
    dbg!(&xs);
    dbg!(&ys);
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

    #[test]
    fn test_star_22() {
        let input = include_str!("assets/day_11_test_input_1.txt");
        let result = star_22(input, 10);
        assert_eq!(result, 1030);
    }
}
