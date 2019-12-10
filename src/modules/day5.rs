use std::error::Error;
use crate::modules::intcode;

pub fn run_part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut machine = intcode::build_intcode_from_input(input)?;
    
    machine.run();

    Ok("Done!".to_string())
}


