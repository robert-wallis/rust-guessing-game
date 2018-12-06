// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

use rand::{Rng, StdRng, SeedableRng, ThreadRng};
use range::Range;

pub trait RandomGenerator {
    fn random(&mut self, range: &Range) -> i32;
}

/// Generator that is seeded or predictable.
pub struct Deterministic {
    random: StdRng,
}

impl Deterministic {
    /// seed example: `let seed &[_] = &[1, 2, 3, 4];`
    pub fn new(seed: &[usize]) -> Deterministic {
        Deterministic {
            random: SeedableRng::from_seed(seed),
        }
    }
}

impl RandomGenerator for Deterministic {
    /// Generate a number between min and max inclusive [min, max].
    fn random(&mut self, range: &Range) -> i32 {
        self.random.gen_range(range.min, range.max + 1)
    }
}

/// Generator that seeded by the OS.
pub struct Randy {
    random: ThreadRng,
}

impl Randy {
    pub fn new() -> Randy {
        Randy {
            random: rand::thread_rng(),
        }
    }
}

impl RandomGenerator for Randy {
    /// Generate a number between min and max inclusive [min, max].
    fn random(&mut self, range: &Range) -> i32 {
        self.random.gen_range(range.min, range.max + 1)
    }
}