use std::fs;
use std::error::Error;


pub fn run(input: &str, part: i32, args: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(input)?;
    let mut intcode: Vec<i32> = contents.split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    if part == 1 {
        intcode[1] = 12;
        intcode[2] = 2;
        let intcode = run_intcode(&mut intcode);
        
        return Ok(format!("Intcode program result: {}", intcode[0]));

    } else {
        let target = args.parse::<i32>().unwrap();

        for noun in 0..100 {
            for verb in 0..100 {
                let mut intcode_copy = intcode.to_vec();
                intcode_copy[1] = noun;
                intcode_copy[2] = verb;

                let result = run_intcode(&mut intcode_copy)[0];
                if result == target {
                    return Ok(format!("Required noun {} and verb {} for target {}", noun, verb, target));
                }
            }
        }
    }

    Err("Could not find noun and verbs for target".into())
}

fn run_intcode(intcode: &mut Vec<i32>) -> &mut Vec<i32> {
    for index in 0..intcode.len() {
        if index % 4 != 0 {
            continue;
        }

        let opcode = intcode[index];
        if opcode == 99 {
            break;
        }

        let first = intcode[index + 1];
        let second = intcode[index + 2];
        let target = intcode[index + 3];

        let result = match opcode {
            1 => intcode[first as usize] + intcode[second as usize],
            2 => intcode[first as usize] * intcode[second as usize],
            _ => { continue; },
        };
        
        intcode[target as usize] = result;
    }

    return intcode;
}

