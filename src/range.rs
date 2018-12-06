// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved

use rand::Rng;

pub struct Range {
    pub min: i32,
    pub max: i32,
}

impl Range {
    /// Generate a number between min and max inclusive [min, max].
    pub fn random(&self) -> i32 {
        let mut random = rand::thread_rng();
        random.gen_range(self.min, self.max + 1)
    }
}
