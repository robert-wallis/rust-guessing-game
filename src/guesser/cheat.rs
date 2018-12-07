// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

use guesser::{self, AskGuessError};
use rand::{Rng, prelude::SeedableRng, prelude::StdRng};
use range::Range;

pub struct CheatGuesser {
    random: StdRng,
}

impl CheatGuesser {
    pub fn new() -> CheatGuesser {
        CheatGuesser {
            random: SeedableRng::seed_from_u64(0x0u64),
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
