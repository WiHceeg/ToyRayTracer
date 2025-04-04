use crate::interval::Interval;

pub const INTENSITY: Interval = Interval::new(0., 0.999);
pub const RAY_MIN_DISTANCE: f64 = 0.0001; //t_min 如果是 0，由于浮点精度的限制，算出一个很小很小的 double，它 > 0，于是继续反射衰减了。但事实上这个解应该是 0，这个解应该舍弃才对，所以设置 t_min 为 0.001，强迫光线走一段路

pub const NEAR_ZERO_THRESHOLD: f64 = 1e-8;

pub const PERLIN_POINT_COUNT: usize = 256;

pub const MINIMUM_AABB_THICKNESS: f64 = 0.0001;