extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guessing Game");

    let min = 1;
    let max = 100;

    let answer = gen_answer(min, max);

    loop {
        let guess = ask_for_guess(min, max);
        return match guess.cmp(&answer) {
            Ordering::Less => println!("Too low."),
            Ordering::Greater => println!("Too high."),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        };
    }

    println!("The answer was: {}", answer);
}

/// Generate a number between min and max inclusive [min, max].
fn gen_answer(min: i32, max: i32) -> i32 {
    let mut random = rand::thread_rng();
    return random.gen_range(min, max + 1);
}

fn ask_for_guess(min: i32, max: i32) -> i32 {
    print!("Guess a number between {}, and {}: ", min, max);
    io::stdout().flush().expect("stdout can't flush");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Read Line Failed :/");

    let guess: i32 = guess.trim().parse().expect("Not a number");
    return guess;
}
