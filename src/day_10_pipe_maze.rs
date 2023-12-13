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
                starting_position = (pos.0 as i32, index as i32);
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
        let max_x = (self.rows[0].len() - 1) as i32;
        let max_y = (self.rows.len() - 1) as i32;
        if result_x < 0 || result_x > max_x {
            return Err(("OOBX", (result_x, result_y)));
        } else if result_y < 0 || result_y > max_y {
            return Err(("OOBY", (result_x, result_y)));
        }
        let char_result = self.rows[result_y as usize][result_x as usize];
        let loc_result = (result_x, result_y);
        match char_result {
            '|' => {
                if direction == &Direction::East || direction == &Direction::West {
                    Err(("Dead End", loc_result))
                } else {
                    Ok((*direction, loc_result))
                }
            }
            '-' => {
                if direction == &Direction::North || direction == &Direction::South {
                    Err(("Dead End", loc_result))
                } else {
                    Ok((*direction, loc_result))
                }
            }
            'L' => match *direction {
                Direction::East | Direction::North => Err(("Dead End", loc_result)),
                Direction::South => Ok((Direction::East, loc_result)),
                Direction::West => Ok((Direction::North, loc_result)),
            },
            'J' => match *direction {
                Direction::West | Direction::North => Err(("Dead End", loc_result)),
                Direction::South => Ok((Direction::West, loc_result)),
                Direction::East => Ok((Direction::North, loc_result)),
            },
            '7' => match *direction {
                Direction::South | Direction::West => Err(("Dead End", loc_result)),
                Direction::North => Ok((Direction::West, loc_result)),
                Direction::East => Ok((Direction::South, loc_result)),
            },
            'F' => match *direction {
                Direction::East | Direction::South => Err(("Dead End", loc_result)),
                Direction::North => Ok((Direction::East, loc_result)),
                Direction::West => Ok((Direction::South, loc_result)),
            },
            '.' => Err(("Empty", loc_result)),
            'S' => Err(("FIN", loc_result)),
            _ => unreachable!(),
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
    star_20(INPUT)
}

fn star_20(input: &str) -> i64 {
    let maze = PipeMaze::from(input);
    let mut starting_direction = Direction::West;
    let mut current_position = maze.starting_position;

    let mut current_direction = starting_direction;
    let mut points = vec![current_position];
    loop {
        let next = maze.next_piece(&current_position, &current_direction);
        match next {
            Ok((direction, pos)) => {
                points.push(pos);
                current_position = pos;
                current_direction = direction;
            }
            Err((str, _pos)) => match str {
                "Dead End" | "OOBX" | "OOBY" | "Empty" => match starting_direction {
                    Direction::North => unreachable!(),
                    Direction::East => {
                        starting_direction = Direction::North;
                        current_direction = starting_direction;
                        current_position = maze.starting_position;
                        points = vec![current_position];
                    }
                    Direction::South => {
                        starting_direction = Direction::East;
                        current_direction = starting_direction;
                        current_position = maze.starting_position;
                        points = vec![current_position];
                    }
                    Direction::West => {
                        starting_direction = Direction::South;
                        current_direction = starting_direction;
                        current_position = maze.starting_position;
                        points = vec![current_position];
                    }
                },
                "FIN" => {
                    break;
                }
                _ => unreachable!(),
            },
        }
    }
    points.push(maze.starting_position);
    let area = shoelace(&points);
    // Picks Theorem
    (area + 1 - points.len() as i32 / 2) as i64
}

fn shoelace(points: &Vec<(i32, i32)>) -> i32 {
    let mut area = 0;
    for index in 0..points.len() - 1 {
        let left = points[index];
        let right = points[index + 1];
        area += left.0 * right.1 - left.1 * right.0;
    }
    area.abs() / 2
}

pub fn exec_star_19() -> i64 {
    star_19(INPUT)
}

fn star_19(input: &str) -> i64 {
    let maze = PipeMaze::from(input);
    let mut starting_direction = Direction::West;
    let mut current_position = maze.starting_position;

    let mut current_direction = starting_direction;
    let mut count = 1;
    loop {
        let next = maze.next_piece(&current_position, &current_direction);
        match next {
            Ok((direction, pos)) => {
                current_position = pos;
                current_direction = direction;
                count += 1;
            }
            Err((str, _pos)) => match str {
                "Dead End" | "OOBX" | "OOBY" | "Empty" => match starting_direction {
                    Direction::North => unreachable!(),
                    Direction::East => {
                        starting_direction = Direction::North;
                        current_direction = starting_direction;
                        current_position = maze.starting_position;
                        count = 1;
                    }
                    Direction::South => {
                        starting_direction = Direction::East;
                        current_direction = starting_direction;
                        current_position = maze.starting_position;
                        count = 1;
                    }
                    Direction::West => {
                        starting_direction = Direction::South;
                        current_direction = starting_direction;
                        current_position = maze.starting_position;
                        count = 1;
                    }
                },
                "FIN" => {
                    break;
                }
                _ => unreachable!(),
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
    #[test]
    fn test_star_20_1() {
        let input = include_str!("assets/day_10_test_input_3.txt");
        let result = star_20(input);
        assert_eq!(result, 4);
    }
}
