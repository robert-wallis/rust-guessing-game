// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

use guesser::{self, AskGuessError};
use range::Range;

const PHI: f32 = 0.618_034;

pub struct PhiGuesser;

impl guesser::Guesser for PhiGuesser {
    fn guess(&mut self, range: &Range) -> Result<i32, AskGuessError> {
        let guess = range.min as f32 + (range.max - range.min) as f32 * PHI;
        Ok(guess as i32)
    }
}
