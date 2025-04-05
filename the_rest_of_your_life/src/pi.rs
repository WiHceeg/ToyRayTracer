mod random_number_generator;

use random_number_generator::{random, random_range};

fn main() {
    let mut inside_circle: u64 = 0;
    let mut inside_circle_stratified: u64 = 0;
    let sqrt_N: u64 = 1000;
    for i in 0..sqrt_N {
        for j in 0..sqrt_N {
            let mut x = random_range(-1.0..1.0);
            let mut y = random_range(-1.0..1.0);
            if x * x + y * y < 1.0 {
                inside_circle += 1;
            }

            x = 2.0 * (i as f64 + random()) / sqrt_N as f64 - 1.0;
            y = 2.0 * (j as f64 + random()) / sqrt_N as f64 - 1.0;
            if x * x + y * y < 1.0 {
                inside_circle_stratified += 1;
            }
        }  
    }

    println!("Regular Estimate of Pi: {}", 4.0 * inside_circle as f64 / (sqrt_N * sqrt_N) as f64);
    println!("Stratified Estimate of Pi: {}", 4.0 * inside_circle_stratified as f64 / (sqrt_N * sqrt_N) as f64);
}
