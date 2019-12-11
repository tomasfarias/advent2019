use std::error::Error;

#[macro_use] extern crate text_io;

extern crate clap;
use clap::{Arg, App, SubCommand, value_t};

mod modules;
pub use modules::day1;
pub use modules::day2;
pub use modules::day3;
pub use modules::day4;
pub use modules::day5;
pub use modules::day6;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("AdventOfCode2019")
        .version("1.0")
        .author("Tomas Farias")
        .subcommand(SubCommand::with_name("day1")
                    .about("Day 1 challenge")
                    .arg(Arg::with_name("input")
                         .short("i")
                         .takes_value(true)
                         .required(true)
                         .help("Sets the path to the challenge input file"))
                    .subcommand(SubCommand::with_name("part1"))
                    .subcommand(SubCommand::with_name("part2")))
        .subcommand(SubCommand::with_name("day2")
                    .about("Day 2 challenge")
                    .arg(Arg::with_name("input")
                         .short("i")
                         .takes_value(true)
                         .required(true)
                         .help("Sets the path to the challenge input file"))
                    .subcommand(SubCommand::with_name("part1")
                                .arg(Arg::with_name("noun")
                                     .short("n")
                                     .takes_value(true)
                                     .required(true)
                                     .help("Sets the intcode machine noun"))
                                .arg(Arg::with_name("verb")
                                     .short("v")
                                     .takes_value(true)
                                     .required(true)
                                     .help("Sets the intcode machine verb")))
                    .subcommand(SubCommand::with_name("part2")
                                .arg(Arg::with_name("target")
                                     .short("t")
                                     .takes_value(true)
                                     .required(true)
                                     .help("Sets the intcode machine target"))))
        .subcommand(SubCommand::with_name("day3")
                    .about("Day 3 challenge")
                    .arg(Arg::with_name("input")
                         .short("i")
                         .takes_value(true)
                         .required(true)
                         .help("Sets the path to the challenge input file"))
                    .subcommand(SubCommand::with_name("part1"))
                    .subcommand(SubCommand::with_name("part2"))) 
        .subcommand(SubCommand::with_name("day4")
                    .about("Day 4 challenge")
                    .arg(Arg::with_name("input")
                         .short("i")
                         .takes_value(true)
                         .required(true)
                         .help("Sets the path to the challenge input file"))
                    .subcommand(SubCommand::with_name("part1"))
                    .subcommand(SubCommand::with_name("part2")))  
        .subcommand(SubCommand::with_name("day5")
                    .about("Day 5 challenge")
                    .arg(Arg::with_name("input")
                         .short("i")
                         .takes_value(true)
                         .required(true)
                         .help("Sets the path to the challenge input file")))   
        .subcommand(SubCommand::with_name("day6")
                    .about("Day 6 challenge")
                    .arg(Arg::with_name("input")
                         .short("i")
                         .takes_value(true)
                         .required(true)
                         .help("Sets the path to the challenge input file"))
                    .subcommand(SubCommand::with_name("part1"))
                    .subcommand(SubCommand::with_name("part2")))  
        .get_matches();

   let result = match matches.subcommand() {
        ("day1", Some(day1_matches)) => {
            let input = day1_matches.value_of("input").unwrap();
            
            match day1_matches.subcommand() {
                ("part1", _) => {
                    day1::run(input, 1).unwrap()
                },
                ("part2", _) => {
                    day1::run(input, 2).unwrap()
                },
                _ => {
                     println!("Unrecognized command or unsolved day 2 challenge part");
                     return Ok(());
                },
            }
        },
        ("day2", Some(day2_matches)) => {
            let input = day2_matches.value_of("input").unwrap();
            
            match day2_matches.subcommand() {
                 ("part1", Some(day2_part1_matches)) => {
                     let noun = value_t!(day2_part1_matches.value_of("noun"), i32).unwrap();
                     let verb = value_t!(day2_part1_matches.value_of("verb"), i32).unwrap();
                     
                     day2::run_part1(input, noun, verb).unwrap()
                 },
                 ("part2", Some(day2_part2_matches)) => {
                     let target = value_t!(day2_part2_matches.value_of("target"), i32).unwrap();

                     day2::run_part2(input, target).unwrap()
                 },
                 _ => {
                     println!("Unrecognized command or unsolved day 2 challenge part");
                     return Ok(());
                },
            }
        },
        ("day3", Some(day3_matches)) => {
            let input = day3_matches.value_of("input").unwrap();

            match day3_matches.subcommand() {
                ("part1", _) => {
                    day3::run_part1(input).unwrap()
                },
                ("part2", _) => {
                    day3::run_part2(input).unwrap()
                },
                _ => {
                     println!("Unrecognized command or unsolved day 3 challenge part");
                     return Ok(());
                },
            }
        },
        ("day4", Some(day4_matches)) => {
            let input = day4_matches.value_of("input").unwrap();

            match day4_matches.subcommand() {
                ("part1", _) => {
                    day4::run_part1(input).unwrap()
                },
                ("part2", _) => {
                    day4::run_part2(input).unwrap()
                },
                _ => {
                     println!("Unrecognized command or unsolved day 4 challenge part");
                     return Ok(());
                },
            }
        },
        ("day5", Some(day5_matches)) => {
            let input = day5_matches.value_of("input").unwrap();
            day5::run(input).unwrap()
        },
        ("day6", Some(day6_matches)) => {
            let input = day6_matches.value_of("input").unwrap();

            match day6_matches.subcommand() {
                ("part1", _) => {
                    day6::run_part1(input).unwrap()
                },
                ("part2", _) => {
                    day6::run_part2(input).unwrap()
                },
                _ => {
                     println!("Unrecognized command or unsolved day 6 challenge part");
                     return Ok(());
                },
            }
        }, 
        _ => {
            println!("Unrecognized command or unsolved day challenge");
            return Ok(());
        },
    };

    println!("{}", result);
    Ok(())
}

