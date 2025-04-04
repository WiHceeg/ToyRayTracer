use crate::color::Color;
use crate::point3::Point3;
use glam::DVec3;

pub const ASPECT_RATIO: f64 = 1.0;
pub const IMAGE_WIDTH: usize = 600;
pub const SAMPLES_PER_PIXEL: usize = 200; // Count of random samples for each pixel
pub const MAX_DEPTH: usize = 50; // Maximum number of ray bounces into scene
pub const BACKGROUND: Color = Color::ZERO;
pub const ENABLE_GRADIENT_SKY: bool = false;

pub const V_FOV: f64 = 40.0;
pub const LOOKFROM: Point3 = Point3::new(278.0, 278.0, -800.0);
pub const LOOKAT: Point3 = Point3::new(278.0, 278.0, 0.0);
pub const V_UP: DVec3 = Point3::new(0.0, 1.0, 0.0);

pub const DEFOCUS_ANGLE: f64 = 0.0;
pub const FOCUS_DIST: f64 = 10.0;
