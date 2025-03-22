use crate::color::Color;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: usize = 400;
pub const SAMPLES_PER_PIXEL: usize = 100; // Count of random samples for each pixel
pub const MAX_DEPTH: usize = 50; // Maximum number of ray bounces into scene

pub const SKY_GRADIENT: Color = Color::new(0.5, 0.7, 1.0);
