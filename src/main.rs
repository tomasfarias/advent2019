use std::error::Error;

extern crate clap;
use clap::{Arg, App, value_t};

mod challenges;
pub use crate::challenges::day1;


fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("AdventOfCode2019")
        .version("1.0")
        .author("Tomas Farias")
        .arg(Arg::with_name("day")
             .short("d")
             .takes_value(true)
             .help("Sets the challenge day")
             .required(true))
        .arg(Arg::with_name("part")
             .short("p")
             .takes_value(true)
             .help("Sets the part of the challenge to complete"))
        .arg(Arg::with_name("input")
             .short("i")
             .takes_value(true)
             .help("Sets the path to the challenge input file"))
        .get_matches();

    let day = value_t!(matches.value_of("day"), i32).unwrap_or_else(|e| e.exit());
    let part = value_t!(matches.value_of("part"), i32).unwrap_or_else(|_| 1);
    let input = matches.value_of("input").unwrap_or_else(|| "");

    let result = match day {
        1 => day1::run(input, part).unwrap(),
        _ => {
            println!("The day {} challenge has not been solved yet", day);
            return Ok(());
        },
    };

    println!("Day {} challenge, part {}\n {}", day, part, result);
    Ok(())
}

