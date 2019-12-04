use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn run(input: &str, part: i32) -> Result<String, io::Error> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let mass = match line?.parse::<i32>() {
            Ok(x) => x,
            Err(_) => 0,
        };
        let mut fuel = mass / 3 - 2;

        if part == 1 {
            total += fuel;
            continue;
        }

        while fuel > 0 {
            total += fuel;
            fuel = fuel / 3 - 2;
        }

    }

    Ok(format!("Total fuel: {}", total))
}
