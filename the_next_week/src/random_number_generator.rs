use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::ops::Range;

thread_local! {
    pub static RNG: RefCell<ThreadRng> = RefCell::new(rand::rng());
}

pub fn get_random_generator() -> ThreadRng {
    RNG.with(|rng| rng.borrow_mut().clone())
}

/// Generates a random number in the range [0, 1)
pub fn random() -> f64{
    RNG.with(|rng| rng.borrow_mut().random())
}

pub fn random_range(range: Range<f64>) -> f64 {
    RNG.with(|rng| rng.borrow_mut().random_range(range))
}
