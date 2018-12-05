// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved
// My version of Guessing Game: https://doc.rust-lang.org/book/2018-edition/ch02-00-guessing-game-tutorial.html

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::convert::From;
use std::fmt;
use std::io;
use std::io::Write;

struct Stats {
    turns: i32,
}

struct Range {
    min: i32,
    max: i32,
}
impl Range {
    /// Generate a number between min and max inclusive [min, max].
    fn random(&self) -> i32 {
        let mut random = rand::thread_rng();
        random.gen_range(self.min, self.max + 1)
    }
}

enum AskGuessError {
    IOError(io::Error),
    NotANumber,
}

trait Guesser {
    fn guess(&self, range: &Range) -> Result<i32, AskGuessError>;
}
trait Displayer {
    fn display_guess_error(&self, err: &AskGuessError);
    fn display_guess_result(&self, result: &GuessResult);
    fn display_stats(&self, stats: &Stats);
}

struct StdIoGuesser;

impl Guesser for StdIoGuesser {
    fn guess(&self, range: &Range) -> Result<i32, AskGuessError> {
        print!("Guess a number between {}, and {}: ", range.min, range.max);
        io::stdout().flush()?;

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;

        match guess.trim().parse() {
            Ok(num) => Ok(num),
            Err(_) => Err(AskGuessError::NotANumber),
        }
    }
}

impl Displayer for StdIoGuesser {
    fn display_guess_error(&self, err: &AskGuessError) {
        println!("{}", err)
    }
    fn display_guess_result(&self, result: &GuessResult) {
        println!("{}", result)
    }
    fn display_stats(&self, stats: &Stats) {
        println!("You figured it out in {} tries.", stats.turns)
    }
}

enum GuessResult {
    TooHigh(i32),
    TooLow(i32),
    Correct(i32),
}

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

impl From<io::Error> for AskGuessError {
    fn from(err: io::Error) -> AskGuessError {
        AskGuessError::IOError(err)
    }
}

impl fmt::Display for AskGuessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AskGuessError::IOError(err) => write!(f, "IO Error: {}", err),
            AskGuessError::NotANumber => write!(f, "Not a valid number."),
        }
    }
}

impl fmt::Display for GuessResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuessResult::TooLow(guess) => write!(f, "{} is too low.", guess),
            GuessResult::TooHigh(guess) => write!(f, "{} is too high.", guess),
            GuessResult::Correct(answer) => write!(f, "{} is correct.", answer),
        }
    }
}

fn main() {
    println!("==== Guessing Game ====");

    let mut msg = Message::GenerateAnswer(Range { min: 1, max: 100 });
    let stdio = StdIoGuesser;
    let guesser: &Guesser = &stdio;
    let displayer: &Displayer = &stdio;

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
    println!("End Game");
}
