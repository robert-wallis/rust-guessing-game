// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

use guesser::{self, AskGuessError};
use range::Range;

pub struct ThirdGuesser;

impl guesser::Guesser for ThirdGuesser {
    fn guess(&mut self, range: &Range) -> Result<i32, AskGuessError> {
        let guess = range.min + (range.max - range.min) / 3;
        Ok(guess)
    }
}
