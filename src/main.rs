extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guessing Game");

    let mut min = 1;
    let mut max = 100;

    let answer = gen_answer(min, max);

    loop {
        let guess = ask_for_guess(min, max);
        match guess {
            Ok(guess) => {
                match guess.cmp(&answer) {
                    Ordering::Less => {
                        println!("Too low.");
                        min = guess + 1;
                    }
                    Ordering::Greater => {
                        println!("Too high.");
                        max = guess - 1;
                    }
                    Ordering::Equal => {
                        println!("You Win!");
                        break;
                    }
                };
            }
            Err(err) => println!("{}", err),
        }
    }

    println!("The answer was: {}", answer);
}

/// Generate a number between min and max inclusive [min, max].
fn gen_answer(min: i32, max: i32) -> i32 {
    let mut random = rand::thread_rng();
    return random.gen_range(min, max + 1);
}

fn ask_for_guess(min: i32, max: i32) -> Result<i32, String> {
    print!("Guess a number between {}, and {}: ", min, max);
    match io::stdout().flush() {
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),
    }

    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),
    }

    return match guess.trim().parse() {
        Ok(num) => Ok(num),
        Err(err) => Err(err.to_string()),
    };
}
