// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

use guesser::{self, AskGuessError};
use range::Range;
use rand::{ Rng, StdRng, SeedableRng };

pub struct CheatGuesser {
    random: StdRng
}

impl CheatGuesser {
    pub fn new() -> CheatGuesser {
        let seed: &[_] = &[1, 2, 3, 4];
        CheatGuesser {
            random: SeedableRng::from_seed(seed),
        }
    }

    pub fn random(&mut self, range: &Range) -> i32 {
        self.random.gen_range(range.min, range.max + 1)
    }
}

impl guesser::Guesser for CheatGuesser {
    fn guess(&mut self, range: &Range) -> Result<i32, AskGuessError> {
        let guess = self.random(range);
        Ok(guess)
    }
}
