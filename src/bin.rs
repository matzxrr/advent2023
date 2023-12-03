use core::panic;

use clap::{Arg, ArgAction, ArgGroup, Command};
use solutions::day_1_trebuchet;

fn cli() -> Command {
    Command::new("advent23")
        .about("Advent of Code 2023 Answer CLI")
        .author("matzxrr")
        .subcommand_required(true)
        .subcommand(
            Command::new("answer")
                .short_flag('a')
                .long_flag("answer")
                .about("Get the answer to a problem")
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
                ),
        )
}

fn get_day_answers(day: &str) {
    println!("Getting answer for day '{}'", day);
}

fn get_star_answer(star: &str) {
    print!("Answer for star '{}' => ", star);
    match star {
        "1" => {
            let answer = day_1_trebuchet::exec_star_1();
            println!("{}", answer);
        }
        _ => panic!("Unknown star"),
    }
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("answer", answer_matches)) => {
            if answer_matches.contains_id("day") {
                let day = answer_matches
                    .get_one::<String>("day")
                    .expect("contains_id");
                get_day_answers(day);
            }
            if answer_matches.contains_id("star") {
                let star = answer_matches
                    .get_one::<String>("star")
                    .expect("contains_id");
                get_star_answer(star);
            }
        }
        _ => unreachable!(),
    }
}
