// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

pub mod aggregator;
pub mod io;

use guesser::{AskGuessError, GuessResult};
use stats::Stats;

pub trait Displayer {
    fn display_guess_error(&self, err: &AskGuessError);
    fn display_guess_result(&self, result: &GuessResult);
    fn display_stats(&mut self, stats: &Stats);
}
