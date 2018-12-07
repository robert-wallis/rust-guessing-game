// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

use guesser::{self, AskGuessError};
use rand::{Rng, prelude::ThreadRng};
use range::Range;

pub struct RandomGuesser {
    random: ThreadRng,
}

impl RandomGuesser {
    pub fn new() -> RandomGuesser {
        RandomGuesser {
            random: rand::thread_rng(),
        }
    }

    pub fn random(&mut self, range: &Range) -> i32 {
        self.random.gen_range(range.min, range.max + 1)
    }
}

impl guesser::Guesser for RandomGuesser {
    fn guess(&mut self, range: &Range) -> Result<i32, AskGuessError> {
        let guess = self.random(range);
        Ok(guess)
    }
}
