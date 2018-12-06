// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

use displayer;
use guesser::{AskGuessError, GuessResult};
use stats::Stats;

pub struct IODisplayer;

impl displayer::Displayer for IODisplayer {
    fn display_guess_error(&self, err: &AskGuessError) {
        println!("{}", err)
    }
    fn display_guess_result(&self, result: &GuessResult) {
        println!("{}", result)
    }
    fn display_stats(&mut self, stats: &Stats) {
        println!("You figured it out in {} tries.", stats.turns)
    }
}
