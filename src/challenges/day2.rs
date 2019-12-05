use std::fs;
use std::error::Error;


pub fn run_part1(input: &str, noun: i32, verb: i32) -> Result<String, Box<dyn Error>> {
    let mut intcode = build_intcode_from_input(input)?;
    
    intcode.set_noun(noun);
    intcode.set_verb(verb);

    intcode.run();
        
    Ok(format!("Intcode program result: {}", intcode.get_result()))
}

pub fn run_part2(input: &str, target: i32) -> Result<String, Box<dyn Error>> {
    let intcode = build_intcode_from_input(input)?;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode_clone = intcode.clone();
            intcode_clone.set_noun(noun);
            intcode_clone.set_verb(verb);

            intcode_clone.run();
            let result = intcode_clone.get_result();
            if result == target {
                return Ok(format!("Required noun {} and verb {} for target {}", noun, verb, target));
            }
        }
    }

    Err("Could not find noun and verbs for target".into())
}

fn build_intcode_from_input(input: &str) -> Result<IntCode, Box<dyn Error>> {
    let contents = fs::read_to_string(input)?;
    let instructions: Vec<i32> = contents.split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    Ok(IntCode {
        instructions: instructions,
        ..Default::default()
    })
}

#[derive(Clone)]
struct IntCode {
    instructions: Vec<i32>,
    state: String,
    index: usize,
}

impl Default for IntCode {
    fn default() -> IntCode {
        IntCode {
            instructions: Vec::new(),
            state: "initialized".to_string(),
            index: 0 as usize,
        }
    }
}

impl IntCode {
    fn next(&mut self) {
        if self.state != "running" && self.state != "initialized" {
            return;
        }

        let opcode = self.instructions[self.index];
        if opcode == 99 {
            self.state = "terminated".to_string();
            return;
        }

        let first = self.instructions[self.index + 1];
        let second = self.instructions[self.index + 2];
        let target = self.instructions[self.index + 3];
    
        let result = match opcode {
            1 => self.instructions[first as usize] + self.instructions[second as usize],
            2 => self.instructions[first as usize] * self.instructions[second as usize],
            _ => { return; },
        };
        
        self.instructions[target as usize] = result;
        self.state = "running".to_string();
        self.index = (self.index + 4) as usize;
    }

    fn run(&mut self) {
        while self.state != "terminated" {
            self.next();
        }
    }

    fn get_result(self) -> i32{
        self.instructions[0 as usize]
    }

    fn set_noun(&mut self, noun: i32) {
        self.instructions[1 as usize] = noun;
    }

    fn set_verb(&mut self, verb: i32) {
        self.instructions[2 as usize] = verb;
    }
}

