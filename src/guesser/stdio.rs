// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

use guesser::{self, AskGuessError, Displayer, GuessResult};
use range::Range;
use stats::Stats;
use std::io;
use std::io::Write;

pub struct StdIoGuesser;

impl guesser::Guesser for StdIoGuesser {
    fn guess(&self, range: &Range) -> Result<i32, AskGuessError> {
        print!("Guess a number between {}, and {}: ", range.min, range.max);
        io::stdout().flush()?;

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;

        match guess.trim().parse() {
            Ok(num) => Ok(num),
            Err(_) => Err(AskGuessError::NotANumber),
        }
    }
}

impl Displayer for StdIoGuesser {
    fn display_guess_error(&self, err: &AskGuessError) {
        println!("{}", err)
    }
    fn display_guess_result(&self, result: &GuessResult) {
        println!("{}", result)
    }
    fn display_stats(&self, stats: &Stats) {
        println!("You figured it out in {} tries.", stats.turns)
    }
}
