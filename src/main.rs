extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guessing Game");

    let min = 1;
    let max = 100;

    let mut random = rand::thread_rng();
    let answer = random.gen_range(min, max + 1);

    print!("Guess a number between {}, and {}: ", min, max);
    io::stdout().flush().expect("stdout can't flush");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Read Line Failed :/");

    let guess: i32 = guess.trim().parse().expect("Not a number");

    match guess.cmp(&answer) {
        Ordering::Less => println!("Too Low"),
        Ordering::Greater => println!("Too High"),
        Ordering::Equal => println!("Yes"),
    }

    println!("The answer was: {}", answer);
}
