// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

pub mod stdio;

use range::Range;
use stats::Stats;
use std::convert::From;
use std::fmt;
use std::io;

pub trait Guesser {
    fn guess(&self, range: &Range) -> Result<i32, AskGuessError>;
}

pub trait Displayer {
    fn display_guess_error(&self, err: &AskGuessError);
    fn display_guess_result(&self, result: &GuessResult);
    fn display_stats(&self, stats: &Stats);
}

pub enum AskGuessError {
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

pub enum GuessResult {
    TooHigh(i32),
    TooLow(i32),
    Correct(i32),
}

impl fmt::Display for GuessResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuessResult::TooLow(guess) => write!(f, "{} is too low.", guess),
            GuessResult::TooHigh(guess) => write!(f, "{} is too high.", guess),
            GuessResult::Correct(answer) => write!(f, "{} is correct.", answer),
        }
    }
}
