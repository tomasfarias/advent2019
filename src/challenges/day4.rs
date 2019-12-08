use std::fs;
use std::error::Error;

pub fn run_part1(input: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(input)?;
    let range: Vec<i32> = contents.split("-")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let start = range[0];
    let end = range[1];
    let valid_passwords = ValidPasswords::new(start);
    let mut count = 0;

    for password in valid_passwords {
        if password > 784956 {
            break;
        }

        count += 1;
    }

    Ok(format!("Valid password count in range {}-{}: {}", start, end, count))
}

// Adapted from https://stackoverflow.com/questions/41536479/splitting-an-integer-into-individual-digits
struct ValidPasswords {
    password: i32,
    divisor: i32,
    current: i32,
}

impl ValidPasswords {
    fn new(password: i32) -> Self {
        let mut divisor = 1;
        while password >= divisor * 10 {
            divisor *= 10;
        }

        ValidPasswords {
            password: password,
            divisor: divisor,
            current: password,
        }
    }
}

impl Iterator for ValidPasswords {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pairs = 0;
        
        while pairs == 0 {
            self.password += 1;
            self.current = self.password;

            let restore_divisor = self.divisor;
            let mut previous_digit = 0;

            while self.divisor != 0 {
                let mut digit = self.current / self.divisor;

                if digit == previous_digit {
                    pairs += 1;
                } else if digit < previous_digit {
                    self.password += (previous_digit - digit) * self.divisor;
                    self.password = self.password / self.divisor * self.divisor;
                    self.current = self.password % self.divisor;
                    
                    pairs += 1;
                    digit = previous_digit;
                }

                previous_digit = digit;
                self.current %= self.divisor;
                self.divisor /= 10;
            }

            self.divisor = restore_divisor;
            self.current = self.password;
        }
        Some(self.password)
    }
    
}
