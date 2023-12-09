const INPUT: &str = include_str!("assets/day_6_input_1.txt");

struct Race {
    time: i32,
    distance: i32,
}

#[derive(Clone, Copy)]
struct RacePermutation {
    race_time_length: i32,
    button_duration: i32,
    distance_traveled: i32,
}

impl RacePermutation {
    fn new(button_duration: i32, race_time_length: i32) -> Self {
        let distance_traveled = button_duration * (race_time_length - button_duration);
        Self {
            race_time_length,
            button_duration,
            distance_traveled,
        }
    }
}

struct TimeCard {
    races: Vec<Race>,
}

impl TimeCard {
    fn get_race_permutations(race_time_length: i32) -> Vec<RacePermutation> {
        let mut race_permutations: Vec<RacePermutation> = vec![];
        for button_held_duration in 0..=race_time_length {
            race_permutations.push(RacePermutation::new(button_held_duration, race_time_length));
        }
        race_permutations
    }

    fn get_winning_permutations(
        race_permutations: &[RacePermutation],
        minimum_distance: i32,
    ) -> impl Iterator<Item = &RacePermutation> {
        race_permutations
            .iter()
            .filter(move |r| r.distance_traveled > minimum_distance)
    }
}

pub fn exec_star_11() -> i32 {
    star_11(INPUT)
}

fn star_11(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_11() {
        let input = include_str!("assets/day_6_test_input_1.txt");
        let result = star_11(input);
        assert_eq!(result, 288);
    }
}
