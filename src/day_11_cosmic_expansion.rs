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

    fn compute_distance(&self, point1: &(i32, i32), point2: &(i32, i32), expand: &i32) -> i64 {
        let (xs, ys) = self.get_expand_map();

        let mut count_x = xs.iter().fold(0, |mut acc, v_usize| {
            let v = *v_usize as i32;
            let (low, hi) = if point1.0 < point2.0 {
                (point1.0, point2.0)
            } else {
                (point2.0, point1.0)
            };
            if low < v && v < hi {
                acc += 1;
            }
            acc
        });
        let mut count_y = ys.iter().fold(0, |mut acc, v_usize| {
            let v = *v_usize as i32;
            let (low, hi) = if point1.1 < point2.1 {
                (point1.1, point2.1)
            } else {
                (point2.1, point1.1)
            };
            if low < v && v < hi {
                acc += 1;
            }
            acc
        });

        count_x *= *expand as i64 - 1;
        count_y *= *expand as i64 - 1;
        let mut distance = ((point1.0 - point2.0).abs() + (point1.1 - point2.1).abs()) as i64;
        distance += count_x + count_y;
        distance
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

pub fn exec_star_22() -> i64 {
    star_22(INPUT, 1000000)
}

fn star_22(input: &str, expand: i32) -> i64 {
    let map = Galaxy::from(input);
    let locations = map.get_locations();
    let mut total: i64 = 0;
    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            let a = locations[i];
            let b = locations[j];
            let distance = map.compute_distance(&a, &b, &expand);
            total += distance;
        }
    }
    total
}

#[cfg(test)]
mod day_11 {
    use super::*;

    const INPUT: &str = include_str!("assets/day_11_test_input_1.txt");
    #[test]
    fn test_star_21() {
        let result = star_21(INPUT);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_star_22_2_expand() {
        let result = star_22(INPUT, 2);
        assert_eq!(result, 374);
    }
    #[test]
    fn test_star_22_10_expand() {
        let result = star_22(INPUT, 10);
        assert_eq!(result, 1030);
    }
    #[test]
    fn test_star_22_100_expand() {
        let result = star_22(INPUT, 100);
        assert_eq!(result, 8410);
    }
}

#[cfg(test)]
mod test_expand {
    use super::*;

    const INPUT: &str = include_str!("assets/day_11_test_input_1.txt");
    #[test]
    fn part_1() {
        let map = Galaxy::from(INPUT);
        let point1 = (7, 1);
        let point2 = (0, 2);
        let distance = map.compute_distance(&point1, &point2, &2);
        assert_eq!(distance, 10);
    }
    #[test]
    fn part_2() {
        let map = Galaxy::from(INPUT);
        let point2 = (7, 1);
        let point1 = (0, 2);
        let distance = map.compute_distance(&point1, &point2, &2);
        assert_eq!(distance, 10);
    }
}
