use std::error::Error;
use crate::modules::intcode;

pub fn run_part1(input: &str, noun: i32, verb: i32) -> Result<String, Box<dyn Error>> {
    let mut machine = intcode::build_intcode_from_input(input)?;
    
    machine.set_noun(noun);
    machine.set_verb(verb);

    machine.run();
        
    Ok(format!("Intcode program result: {}", machine.get_result()))
}

pub fn run_part2(input: &str, target: i32) -> Result<String, Box<dyn Error>> {
    let machine = intcode::build_intcode_from_input(input)?;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut machine_clone = machine.clone();
            machine_clone.set_noun(noun);
            machine_clone.set_verb(verb);

            machine_clone.run();
            let result = machine_clone.get_result();
            if result == target {
                return Ok(format!("Required noun {} and verb {} for target {}", noun, verb, target));
            }
        }
    }

    Err("Could not find noun and verbs for target".into())
}

