use crate::color::Color;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: usize = 400;
pub const SAMPLES_PER_PIXEL: usize = 100; // Count of random samples for each pixel
pub const MAX_DEPTH: usize = 50; // Maximum number of ray bounces into scene

pub const SKY_GRADIENT: Color = Color::new(0.5, 0.7, 1.0);
pub const RAY_MIN_DISTANCE: f64 = 0.001; //t_min 如果是 0，由于浮点精度的限制，算出一个很小很小的 double，它 > 0，于是继续反射衰减了。但事实上这个解应该是 0，这个解应该舍弃才对，所以设置 t_min 为 0.001，强迫光线走一段路
