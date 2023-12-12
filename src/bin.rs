use clap::{Arg, ArgAction, ArgGroup, Command};
use solutions::{
    day_1_trebuchet, day_2_cube_conundrum, day_3_gear_ratios, day_4_scratchcards,
    day_5_if_you_give_a_seed_a_fertilizer, day_6_wait_for_it, day_7_camel_cards,
    day_8_haunted_wasteland,
};

fn cli() -> Command {
    Command::new("advent23")
        .about("Advent of Code 2023 Answer CLI")
        .author("matzxrr")
        .group(ArgGroup::new("ans").required(true).multiple(false))
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .help("Day you want the answers for")
                .group("ans")
                .conflicts_with("star")
                .action(ArgAction::Set)
                .num_args(1..),
        )
        .arg(
            Arg::new("star")
                .short('s')
                .long("star")
                .help("Star you want the answer for")
                .group("ans")
                .conflicts_with("day")
                .action(ArgAction::Set)
                .num_args(1..),
        )
}

fn get_day_answers(day: &str) {
    let day_as_int = day.parse::<i32>().unwrap();
    let second = day_as_int * 2;
    let first = second - 1;

    get_star_answer(&first.to_string());
    get_star_answer(&second.to_string());
}

fn get_star_answer(star: &str) {
    let answer_is_u64 = vec!["16"];
    if answer_is_u64.contains(&star) {
        let answer = match star {
            "16" => day_8_haunted_wasteland::exec_star_16(),
            _ => {
                eprintln!("Unknown star '{}'", star);
                return;
            }
        };
        println!("Answer for star '{}' => {}", star, answer);
        return;
    }

    let answer = match star {
        "1" => day_1_trebuchet::exec_star_1(),
        "2" => day_1_trebuchet::exec_star_2(),
        "3" => day_2_cube_conundrum::exec_star_3(),
        "4" => day_2_cube_conundrum::exec_star_4(),
        "5" => day_3_gear_ratios::exec_star_5(),
        "6" => day_3_gear_ratios::exec_star_6(),
        "7" => day_4_scratchcards::exec_star_7(),
        "8" => day_4_scratchcards::exec_star_8(),
        "9" => day_5_if_you_give_a_seed_a_fertilizer::exec_star_9(),
        "10" => day_5_if_you_give_a_seed_a_fertilizer::exec_star_10(),
        "11" => day_6_wait_for_it::exec_star_11(),
        "12" => day_6_wait_for_it::exec_star_12(),
        "13" => day_7_camel_cards::exec_star_13(),
        "14" => day_7_camel_cards::exec_star_14(),
        "15" => day_8_haunted_wasteland::exec_star_15(),
        _ => {
            eprintln!("Unknown star '{}'", star);
            return;
        }
    };
    println!("Answer for star '{}' => {}", star, answer);
}

fn main() {
    let matches = cli().get_matches();
    if matches.contains_id("day") {
        let day = matches.get_one::<String>("day").expect("contains_id");
        get_day_answers(day);
    }
    if matches.contains_id("star") {
        let star = matches.get_one::<String>("star").expect("contains_id");
        get_star_answer(star);
    }
}
