use core::fmt;

const INPUT: &str = include_str!("assets/day_10_input_1.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct PipeMaze {
    rows: Vec<Vec<char>>,
    starting_position: (i32, i32),
}

impl From<&str> for PipeMaze {
    fn from(value: &str) -> Self {
        let mut starting_position = (0, 0);
        let mut rows = Vec::new();
        for (index, line) in value.lines().enumerate() {
            let chars = line.chars();
            rows.push(chars.clone().collect());
            if let Some(pos) = chars.enumerate().find(|c| c.1 == 'S') {
                starting_position = (index as i32, pos.0 as i32);
            }
        }
        Self {
            rows,
            starting_position,
        }
    }
}

type NextPieceResult = Result<(Direction, (i32, i32)), (&'static str, (i32, i32))>;

impl PipeMaze {
    fn next_piece(&self, starting_position: &(i32, i32), direction: &Direction) -> NextPieceResult {
        let dir = PipeMaze::direction(direction);
        let result_x = starting_position.0 + dir.0;
        let result_y = starting_position.1 + dir.1;
        let max_x = (self.rows.len() - 1) as i32;
        let max_y = (self.rows[0].len() - 1) as i32;
        if result_x < 0 || result_x > max_x {
            return Err(("OOBX", (result_x, result_y)));
        } else if result_y < 0 || result_y > max_y {
            return Err(("OOBY", (result_x, result_y)));
        }
        let char_result = self.rows[result_y as usize][result_x as usize];
        let next_direction = match char_result {
            '|' => Some(*direction),
            '-' => Some(*direction),
            'L' => {
                if direction == &Direction::South {
                    Some(Direction::East)
                } else {
                    Some(Direction::North)
                }
            }
            'J' => {
                if direction == &Direction::South {
                    Some(Direction::West)
                } else {
                    Some(Direction::North)
                }
            }
            '7' => {
                if direction == &Direction::East {
                    Some(Direction::South)
                } else {
                    Some(Direction::West)
                }
            }
            'F' => {
                if direction == &Direction::West {
                    Some(Direction::South)
                } else {
                    Some(Direction::East)
                }
            }
            '.' => None,
            'S' => None,
            _ => unreachable!(),
        };

        if let Some(next_dir) = next_direction {
            Ok((next_dir, (result_x, result_y)))
        } else if char_result == '.' {
            Err(("No Pipe", (result_x, result_y)))
        } else {
            Err(("End", (result_x, result_y)))
        }
    }

    fn direction(direction: &Direction) -> (i32, i32) {
        match direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
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

    let mut current_position = maze.starting_position;
    let mut current_direction = Direction::South;
    let mut count = 1;
    loop {
        let next = maze.next_piece(&current_position, &current_direction);
        match next {
            Ok((direction, pos)) => {
                println!(
                    "headed {} from position ({}, {})",
                    &direction, &pos.0, &pos.1
                );
                current_position = pos;
                current_direction = direction;
                count += 1;
            }
            Err((str, pos)) => match str {
                "No Pipe" => {}
                _ => break,
            },
        }
    }

    count / 2
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
