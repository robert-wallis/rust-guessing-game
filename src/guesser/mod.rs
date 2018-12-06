// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

pub mod cheat;
pub mod half;
pub mod io;
pub mod phi;
pub mod random;
pub mod third;

use range::Range;
use std;
use std::convert::From;
use std::fmt;

pub trait Guesser {
    fn guess(&mut self, range: &Range) -> Result<i32, AskGuessError>;
}

pub enum AskGuessError {
    IOError(std::io::Error),
    NotANumber,
}

impl From<std::io::Error> for AskGuessError {
    fn from(err: std::io::Error) -> AskGuessError {
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
