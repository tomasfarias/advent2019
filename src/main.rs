use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("input/mass.txt")?;
    let reader = BufReader::new(file);
    let mut running_total = 0; 

    for line in reader.lines() {
        let mass = match line?.parse::<i32>() { 
            Ok(x) => x,
            Err(_) => 0,
        };
        let mut fuel = fuel_counter_upper(mass);

        while fuel > 0 {
            running_total += fuel;
            fuel = fuel_counter_upper(fuel);
        }

    }

    println!("Total fuel: {}", running_total);
    Ok(())
}

fn fuel_counter_upper(mass: i32) -> i32{ 
    mass / 3 - 2
}



