const INPUT: &str = include_str!("assets/day_6_input_1.txt");

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code, unused)]
struct RacePermutation {
    race_time_length: u64,
    button_duration: u64,
    distance_traveled: u64,
}

impl RacePermutation {
    fn new(button_duration: u64, race_time_length: u64) -> Self {
        let distance_traveled = button_duration * (race_time_length - button_duration);
        Self {
            race_time_length,
            button_duration,
            distance_traveled,
        }
    }
}

#[derive(Debug)]
struct TimeCard {}

impl TimeCard {
    fn parse_lines_into_races(input: &str) -> Vec<Race> {
        let mut parsed_lines = vec![];
        for line in input.lines() {
            let (_, right) = line.split_once(':').unwrap();
            let times: Vec<_> = right
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            parsed_lines.push(times);
        }
        let races: Vec<Race> = (0..parsed_lines[0].len())
            .map(|n| Race {
                time: parsed_lines[0][n],
                distance: parsed_lines[1][n],
            })
            .collect();
        races
    }
    fn parse_lines_into_race(input: &str) -> Race {
        let parts: Vec<u64> = input
            .lines()
            .map(|n| {
                let (_, right) = n.split_once(':').unwrap();
                let num = right
                    .split(' ')
                    .filter(|t| !t.is_empty())
                    .collect::<Vec<_>>()
                    .join("");
                num.parse::<u64>().unwrap()
            })
            .collect();
        Race {
            time: parts[0],
            distance: parts[1],
        }
    }
    fn get_race_permutations(race_time_length: u64) -> Vec<RacePermutation> {
        let mut index = 0;
        let mut rp: Vec<_> = vec![];
        while index <= race_time_length {
            rp.push(RacePermutation::new(index, race_time_length));
            index += 1;
        }
        rp
    }

    fn get_winning_permutations(
        race_permutations: &[RacePermutation],
        minimum_distance: u64,
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
    let races = TimeCard::parse_lines_into_races(input);
    let mut result: i32 = 1;
    for race in races {
        let race_permutations = TimeCard::get_race_permutations(race.time);
        let winning_permutations =
            TimeCard::get_winning_permutations(&race_permutations, race.distance);
        result *= winning_permutations.count() as i32;
    }
    result
}

pub fn exec_star_12() -> i32 {
    star_12(INPUT)
}

fn star_12(input: &str) -> i32 {
    let race = TimeCard::parse_lines_into_race(input);
    let rp = TimeCard::get_race_permutations(race.time);
    let wp = TimeCard::get_winning_permutations(&rp, race.distance);
    wp.count() as i32
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

    #[test]
    fn test_star_12() {
        let input = include_str!("assets/day_6_test_input_1.txt");
        let result = star_12(input);
        assert_eq!(result, 71503);
    }
}
