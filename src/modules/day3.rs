use std::fs;
use std::error::Error;
use std::collections::HashMap;
use std::io;

use itertools::{Itertools, EitherOrBoth::*};


struct Wire {
    coordinates: Coordinates,
    history: HashMap<Coordinates, i32>,
}

impl Wire {
    fn new() -> Wire {
        let start_coordinates = Coordinates { x: 0, y: 0 };
        let start_history = HashMap::new();

        Wire {
            coordinates: start_coordinates,
            history: start_history,
        }
    }

    fn move_command(&mut self, direction: &str, amount: i32, start: i32) {
        match direction {
            "R" => self.move_right(amount, start),
            "L" => self.move_left(amount, start),
            "U" => self.move_up(amount, start),
            "D" => self.move_down(amount, start),
            _ => {},
        }
    }

    fn move_right(&mut self, amount: i32, start: i32) {
        for steps in 1..(amount + 1) {
            self.coordinates.x += 1;
            
            self.history.entry(self.coordinates).or_insert(steps + start);
        }
    }

   
    fn move_left(&mut self, amount: i32, start: i32) {
        for steps in 1..(amount + 1) {
            self.coordinates.x -= 1;
            
            self.history.entry(self.coordinates).or_insert(steps + start);
        }
    }

    fn move_up(&mut self, amount: i32, start: i32) {
        for steps in 1..(amount + 1) {
            self.coordinates.y += 1;

            self.history.entry(self.coordinates).or_insert(steps + start);
        }
    }

    fn move_down(&mut self, amount: i32, start: i32) {
        for steps in 1..(amount + 1) {
            self.coordinates.y -= 1;

            self.history.entry(self.coordinates).or_insert(steps + start);
        }
    }
}


#[derive(Debug, Copy, Clone, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Coordinates {}

impl Coordinates {
    fn manhattan_distance(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub fn run_part1(input: &str) -> Result<String, Box<dyn Error>> {
    let instructions = read_instructions(input).unwrap();
    let (wire1, wire2) = run_wires(instructions);

    let center = Coordinates { x: 0, y: 0 };
    let mut current_min = 0;
    
    for coordinate in wire2.history
        .keys()
        .filter(|k| wire1.history.contains_key(k)) {

        if *coordinate == center {
            continue;
        }

        let distance = center.manhattan_distance(*coordinate);
        if current_min == 0 || distance < current_min {
            current_min = distance;
        }
    }

    Ok(format!("The minimun distance is {}", current_min))
}

pub fn run_part2(input: &str) -> Result<String, Box<dyn Error>> {
    let instructions = read_instructions(input).unwrap();
    let (wire1, wire2) = run_wires(instructions);

    let center = Coordinates { x: 0, y: 0 };
    let mut current_min = 0;

    for (coordinate, steps) in wire2.history
        .iter()
        .filter(|(k, _)| wire1.history.contains_key(k)) {
        if *coordinate == center {
            continue;
        }

        let total_steps = wire1.history.get(&coordinate).unwrap() + *steps;
        if current_min == 0 || total_steps < current_min {
            current_min = total_steps;
        }
    }

    Ok(format!("The minimum amount of steps to reach an intersection is {}", current_min))
}

fn read_instructions(input: &str) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(input)?;
    let instructions: Vec<String> = contents.split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(instructions)
}


fn run_wires(mut instructions: Vec<String>) -> (Wire, Wire) {
    let wire1_instructions = instructions.pop().unwrap();
    let wire2_instructions = instructions.pop().unwrap();

    let wire1_moves = wire1_instructions.split(",");
    let wire2_moves = wire2_instructions.split(",");

    let mut wire1 = Wire::new();
    let mut wire2 = Wire::new();

    let mut start1 = 0;
    let mut start2 = 0;
    
    let iter_moves = wire1_moves.zip_longest(wire2_moves); 

    for moves in iter_moves {
        match moves {
            Both(w1, w2) => {
                wire1.move_command(
                    &w1[..1],
                    w1[1..].parse::<i32>().unwrap(),
                    start1,
                );
                wire2.move_command(
                    &w2[..1],
                    w2[1..].parse::<i32>().unwrap(),
                    start2,
                );

                start1 += w1[1..].parse::<i32>().unwrap();
                start2 += w2[1..].parse::<i32>().unwrap();
            },
            Left(w1) => {
                wire1.move_command(
                    &w1[..1],
                    w1[1..].parse::<i32>().unwrap(),
                    start1
                );

                start1 += w1[1..].parse::<i32>().unwrap();
            },
            Right(w2) => {
                wire2.move_command(
                    &w2[..1],
                    w2[1..].parse::<i32>().unwrap(),
                    start2,
                );

                start2 += w2[1..].parse::<i32>().unwrap();
            },
        }
    }

    (wire1, wire2)
}
