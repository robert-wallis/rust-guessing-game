// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved
// My version of Guessing Game: https://doc.rust-lang.org/book/2018-edition/ch02-00-guessing-game-tutorial.html

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::convert::From;
use std::fmt;
use std::io;
use std::io::Write;

struct Stats {
    turns: i32,
}

struct GameState {
    min: i32,
    max: i32,
    answer: i32,
    stats: Stats,
}

enum AskGuessError {
    IOError(io::Error),
    NotANumber,
}

impl From<io::Error> for AskGuessError {
    fn from(err: io::Error) -> AskGuessError {
        AskGuessError::IOError(err)
    }
}

impl fmt::Display for AskGuessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AskGuessError::IOError(err) => write!(f, "IO Error: {}", err),
            AskGuessError::NotANumber => write!(f, "Not a valid number."),
        }
    }
}

fn main() {
    println!("Guessing Game");

    let mut game_state = GameState {
        min: 1,
        max: 100,
        answer: 0,
        stats: Stats { turns: 0 },
    };

    game_state.answer = gen_answer(game_state.min, game_state.max);

    loop {
        game_state.stats.turns += 1;
        let guess = ask_for_guess(game_state.min, game_state.max);
        match guess {
            Ok(guess) => {
                match guess.cmp(&game_state.answer) {
                    Ordering::Less => {
                        println!("Too low.");
                        game_state.min = guess + 1;
                    }
                    Ordering::Greater => {
                        println!("Too high.");
                        game_state.max = guess - 1;
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

    println!(
        "The answer was: {}, you got it in {} turns.",
        game_state.answer, game_state.stats.turns
    );
}

/// Generate a number between min and max inclusive [min, max].
fn gen_answer(min: i32, max: i32) -> i32 {
    let mut random = rand::thread_rng();
    random.gen_range(min, max + 1)
}

fn ask_for_guess(min: i32, max: i32) -> Result<i32, AskGuessError> {
    print!("Guess a number between {}, and {}: ", min, max);
    io::stdout().flush()?;

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)?;

    match guess.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err(AskGuessError::NotANumber),
    }
}
