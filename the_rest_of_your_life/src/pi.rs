mod random_number_generator;

use random_number_generator::random_range;

fn main() {
    let mut inside_circle: u64 = 0;
    let mut runs: u64 = 0;
    loop {
        runs += 1;
        let x = random_range(-1.0..1.0);
        let y = random_range(-1.0..1.0);
        if x * x + y * y < 1.0 {
            inside_circle += 1;
        }
        if runs % 1000000 == 0 {
            let pi = 4.0 * inside_circle as f64 / runs as f64;
            print!("\rEstimated of Pi after {} runs: {:.12}", runs, pi);
        }
    }

}
