use std::fs;
use std::error::Error;
use std::collections::HashSet;

use itertools::{Itertools, EitherOrBoth::*};


struct Wire {
    coordinates: Coordinates,
    history: HashSet<Coordinates>,
}

impl Wire {
    fn new() -> Wire {
        let start_coordinates = Coordinates { x: 0, y: 0 };
        let start_history = HashSet::new();

        Wire {
            coordinates: start_coordinates,
            history: start_history,
        }
    }

    fn move_command(&mut self, command: &str) {
        let amount = command[1..].parse::<i32>().unwrap();
        match &command[..1] {
            "R" => self.move_right(amount),
            "L" => self.move_left(amount),
            "U" => self.move_up(amount),
            "D" => self.move_down(amount),
            _ => {},
        }
    }

    fn move_right(&mut self, amount: i32) {
        for _ in 1..(amount + 1) {
            self.coordinates.x += 1;
            self.history.insert(self.coordinates.clone());
        }
    }

   
    fn move_left(&mut self, amount: i32) {
        for _ in 1..(amount + 1) {
            self.coordinates.x -= 1;
            self.history.insert(self.coordinates.clone());
        }
    }

    fn move_up(&mut self, amount: i32) {
        for _ in 1..(amount + 1) {
            self.coordinates.y += 1;
            self.history.insert(self.coordinates.clone());
        }
    }

    fn move_down(&mut self, amount: i32) {
        for _ in 1..(amount + 1) {
            self.coordinates.y -= 1;
            self.history.insert(self.coordinates.clone());
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
    let contents = fs::read_to_string(input)?;
    let mut instructions: Vec<&str> = contents.split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let wire1_moves = instructions.pop().unwrap().split(",");
    let wire2_moves = instructions.pop().unwrap().split(",");

    let mut wire1 = Wire::new();
    let mut wire2 = Wire::new();

    for moves in wire1_moves.zip_longest(wire2_moves) {
        match moves {
            Both(w1, w2) => {
                wire1.move_command(w1);
                wire2.move_command(w2);
            },
            Left(w1) => {
                wire1.move_command(w1);
            },
            Right(w2) => {
                wire2.move_command(w2);
            },
        }
    }

    let matches: Vec<Coordinates> = wire2
        .history
        .into_iter()
        .filter(|c| wire1.history.contains(&c))
        .collect();
    
    let center = Coordinates { x: 0, y: 0 };
    let mut current_min = 0;
    
    for coordinate in matches { 
        if coordinate == center {
            continue;
        }

        let distance = center.manhattan_distance(coordinate);
        if current_min == 0 || distance < current_min {
            current_min = distance;
        }
    }

    Ok(format!("The minimun distance is {}", current_min))
}


