// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved
// My version of Guessing Game: https://doc.rust-lang.org/book/2018-edition/ch02-00-guessing-game-tutorial.html

extern crate rand;

mod guesser;
use guesser::{GuessResult, Guesser, io::IOGuesser, half::HalfGuesser};
mod displayer;
use displayer::{Displayer, io::IODisplayer};
mod range;
use range::Range;
mod stats;
use stats::Stats;

use std::cmp::Ordering;

enum Message {
    GenerateAnswer(Range),
    AskForGuess {
        answer: i32,
        range: Range,
        stats: Stats,
    },
    CheckGuess {
        guess: i32,
        answer: i32,
        range: Range,
        stats: Stats,
    },
    ShowGuessResult {
        result: GuessResult,
        answer: i32,
        range: Range,
        stats: Stats,
    },
    ShowStats {
        stats: Stats,
    },
}

fn main() {
    println!("==== Guessing Game ====");

    let _io_guesser = IOGuesser;
    let io_displayer = IODisplayer;
    let half = HalfGuesser;

    //gameloop(Range { min: 1, max: 100 }, &half, &io_displayer);
    gameloop(Range { min: 1, max: 100 }, &half, &io_displayer);

    println!("End Game");
}

fn gameloop(range: Range, guesser: &Guesser, displayer: &Displayer) {
    let mut msg = Message::GenerateAnswer(range);
    loop {
        msg = match msg {
            Message::GenerateAnswer(range) => Message::AskForGuess {
                answer: range.random(),
                range,
                stats: Stats { turns: 0 },
            },
            Message::AskForGuess {
                answer,
                range,
                stats,
            } => match guesser.guess(&range) {
                Ok(guess) => Message::CheckGuess {
                    guess,
                    answer,
                    range,
                    stats,
                },
                Err(err) => {
                    displayer.display_guess_error(&err);
                    Message::AskForGuess {
                        answer,
                        range,
                        stats,
                    }
                }
            },
            Message::CheckGuess {
                guess,
                answer,
                range,
                stats,
            } => {
                let stats = Stats {
                    turns: stats.turns + 1,
                };
                match guess.cmp(&answer) {
                    Ordering::Less => Message::ShowGuessResult {
                        result: GuessResult::TooLow(guess),
                        answer,
                        range: Range {
                            min: guess + 1,
                            ..range
                        },
                        stats,
                    },
                    Ordering::Greater => Message::ShowGuessResult {
                        result: GuessResult::TooHigh(guess),
                        answer,
                        range: Range {
                            max: guess - 1,
                            ..range
                        },
                        stats,
                    },
                    Ordering::Equal => {
                        let result = GuessResult::Correct(answer);
                        Message::ShowGuessResult {
                            result,
                            answer,
                            range,
                            stats,
                        }
                    }
                }
            }
            Message::ShowGuessResult {
                result,
                answer,
                range,
                stats,
            } => {
                displayer.display_guess_result(&result);
                match result {
                    GuessResult::Correct(_) => Message::ShowStats { stats },
                    _ => Message::AskForGuess {
                        answer,
                        range,
                        stats,
                    },
                }
            }
            Message::ShowStats { stats } => {
                displayer.display_stats(&stats);
                break;
            }
        }
    }
}
