mod random_number_generator;

use random_number_generator::random_range;

fn main() {
    let N = 100000000;
    let mut inside_circle = 0;
    for _ in 0..N {
        let x = random_range(-1.0..1.0);
        let y = random_range(-1.0..1.0);
        if x * x + y * y < 1.0 {
            inside_circle += 1;
        }
    }
    let pi = 4.0 * inside_circle as f64 / N as f64;
    println!("Estimated value of pi: {:.12}", pi);
}
