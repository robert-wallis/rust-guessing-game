// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

use displayer;
use guesser::{AskGuessError, GuessResult};
use stats::Stats;

pub struct Aggregator {
    pub most_turns: u32,
    pub least_turns: u32,
    pub average_turns: f32,
    pub games_played: u32,
}

impl Aggregator {
    pub fn new() -> Aggregator {
        Aggregator {
            most_turns: 0,
            least_turns: 0,
            average_turns: 0.0,
            games_played: 0
        }
    }

    fn played(&mut self, turns: u32) {
        self.most_turns = if turns > self.most_turns {turns} else {self.most_turns};
        self.least_turns = if turns < self.least_turns || self.least_turns == 0 {turns} else {self.least_turns};
        self.average_turns = ((self.games_played as f32 * self.average_turns) + turns as f32) / (self.games_played as f32 + 1.0);
        self.games_played += 1;
    }
}

impl displayer::Displayer for Aggregator {
    fn display_guess_error(&self, err: &AskGuessError) {
        println!("{}", err)
    }
    fn display_guess_result(&self, _result: &GuessResult) {
    }
    fn display_stats(&mut self, stats: &Stats) {
        self.played(stats.turns);
    }
}