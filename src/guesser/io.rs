// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

use guesser::{self, AskGuessError};
use range::Range;
use std::io;
use std::io::Write;

pub struct IOGuesser;

impl guesser::Guesser for IOGuesser {
    fn guess(&mut self, range: &Range) -> Result<i32, AskGuessError> {
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
