// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved
// My version of Guessing Game: https://doc.rust-lang.org/book/2018-edition/ch02-00-guessing-game-tutorial.html

extern crate rand;

mod guesser;
use guesser::{
    cheat::CheatGuesser, half::HalfGuesser, io::IOGuesser, phi::PhiGuesser, random::RandomGuesser,
    third::ThirdGuesser, GuessResult, Guesser,
};
mod displayer;
use displayer::{aggregator::Aggregator, io::IODisplayer, Displayer};
mod range;
use range::Range;
mod stats;
use stats::Stats;
mod randy;
use randy::{Deterministic, RandomGenerator, Randy};

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

    test_human();
    test_guesser("Half", &mut HalfGuesser);
    test_guesser("Third", &mut ThirdGuesser);
    test_guesser("Phi", &mut PhiGuesser);
    test_guesser("Random", &mut RandomGuesser::new());
    test_guesser("Cheat", &mut CheatGuesser::new());

    println!("\nEnd Game");
}

fn test_guesser(name: &str, guesser: &mut Guesser) {
    let mut aggregator = Aggregator::new();
    let seed: &[_] = &[1, 2, 3, 4];
    let mut random = Deterministic::new(seed);
    println!("\nTesting {} Guesser", name);
    for _ in 0..1_000_000 {
        message_pump(
            Range { min: 1, max: 100 },
            guesser,
            &mut aggregator,
            &mut random,
        );
    }
    println!(
        "min turns: {}\nmax turns: {}\naverage turns: {}\ngames played: {}",
        aggregator.least_turns,
        aggregator.most_turns,
        aggregator.average_turns,
        aggregator.games_played
    );
}

fn test_human() {
    println!("\nTesting Human");
    let mut random = Randy::new();
    message_pump(
        Range { min: 1, max: 100 },
        &mut IOGuesser,
        &mut IODisplayer,
        &mut random,
    );
}

fn message_pump(
    range: Range,
    guesser: &mut Guesser,
    displayer: &mut Displayer,
    random: &mut RandomGenerator,
) {
    let mut msg = Message::GenerateAnswer(range);
    loop {
        msg = match msg {
            Message::GenerateAnswer(range) => Message::AskForGuess {
                answer: random.random(&range),
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
                            min: if range.min < guess + 1 {
                                guess + 1
                            } else {
                                range.min
                            },
                            ..range
                        },
                        stats,
                    },
                    Ordering::Greater => Message::ShowGuessResult {
                        result: GuessResult::TooHigh(guess),
                        answer,
                        range: Range {
                            max: if range.max > guess - 1 {
                                guess - 1
                            } else {
                                range.max
                            },
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
                    _ => {
                        if range.min == range.max {
                            // if there's only one possiblity, then enter it for them
                            Message::CheckGuess {
                                guess: range.min,
                                answer,
                                range,
                                stats,
                            }
                        } else {
                            Message::AskForGuess {
                                answer,
                                range,
                                stats,
                            }
                        }
                    }
                }
            }
            Message::ShowStats { stats } => {
                displayer.display_stats(&stats);
                break;
            }
        }
    }
}
