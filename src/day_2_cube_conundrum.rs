use std::collections::HashMap;

const INPUT_TO_PARSE: &str = include_str!("assets/day_2_input_1.txt");

pub fn exec_star_3() -> i32 {
    star_3(INPUT_TO_PARSE)
}

fn star_3(input: &str) -> i32 {
    let cubes: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut total = 0;
    'main: for line in input.lines() {
        let (game_string, game_data) = line.rsplit_once(':').unwrap();
        let game_id = game_string.rsplit_once(' ').unwrap().1;
        let turns = game_data.trim().split(';');
        for turn in turns {
            let cube_groups: Vec<_> = turn.trim().split(',').collect();
            for cube_group in cube_groups {
                let cube_group_parts: Vec<_> = cube_group.trim().split(' ').collect();
                let cube_count = cube_group_parts[0].parse::<i32>().unwrap();
                let cube_color = cube_group_parts[1];
                let max = cubes.get(cube_color).unwrap();
                if cube_count > *max {
                    continue 'main; // game is bad
                }
            }
        }
        // game is good if we reached this line
        total += game_id.parse::<i32>().unwrap();
    }
    total
}

pub fn exec_star_2() -> i32 {
    star_4(INPUT_TO_PARSE)
}

fn star_4(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("assets/day_2_test_input_1.txt");

    #[test]
    fn test_star_3() {
        let result = star_3(TEST_INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_star_4() {
        let result = star_4(TEST_INPUT);
        assert_eq!(result, 2286);
    }
}
