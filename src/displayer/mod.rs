// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

pub mod io;

use stats::Stats;
use guesser::{AskGuessError, GuessResult};

pub trait Displayer {
    fn display_guess_error(&self, err: &AskGuessError);
    fn display_guess_result(&self, result: &GuessResult);
    fn display_stats(&self, stats: &Stats);
}
