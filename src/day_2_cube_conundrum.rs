use std::collections::HashMap;

const INPUT_TO_PARSE: &str = include_str!("assets/star_3_input.txt");

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
