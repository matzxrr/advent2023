use clap::{Arg, ArgAction, ArgGroup, Command};
use solutions::day_1_trebuchet;

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
    match day {
        "1" => {
            get_star_answer("1");
            get_star_answer("2");
        }
        _ => eprintln!("Unknown day '{}'", day),
    }
}

fn get_star_answer(star: &str) {
    match star {
        "1" => {
            let answer = day_1_trebuchet::exec_star_1();
            println!("Answer for star '{}' => {}", star, answer);
        }
        "2" => {
            let answer = day_1_trebuchet::exec_star_2();
            println!("Answer for star '{}' => {}", star, answer);
        }
        _ => eprintln!("Unknown star '{}'", star),
    }
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
