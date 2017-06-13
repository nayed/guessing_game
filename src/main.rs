extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

#[derive(Debug, Eq)]
struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}",
                   value);
        }

        Guess { value: value }
    }
}

impl Ord for Guess {
    fn cmp(&self, other: &Guess) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Guess {
    fn partial_cmp(&self, other: &Guess) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Guess {
    fn eq(&self, other: &Guess) -> bool {
        self.value == other.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be greater or equal to 1")]
    fn less_than_1() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

fn main() {
    println!("Yo guess the number!");

    let secret_number = Guess::new(rand::thread_rng().gen_range(1, 101));

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Yay you won!");
                break;
            }
        }
    }
}
