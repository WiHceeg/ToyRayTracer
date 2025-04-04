use std::ops::Range;

use rand::rngs::ThreadRng;

pub fn get_random_generator() -> ThreadRng {
    rand::rng()
}

/// Generates a random number in the range [0, 1)
pub fn random() -> f64{
    rand::random()
}

pub fn random_range(range: Range<f64>) -> f64 {
    rand::random_range(range)
}
