use crate::color::Color;
use crate::point3::Point3;
use glam::DVec3;

pub const INPUT_POINT_SCALE: f64 = 4.0;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: usize = 400;
pub const SAMPLES_PER_PIXEL: usize = 100; // Count of random samples for each pixel
pub const MAX_DEPTH: usize = 50; // Maximum number of ray bounces into scene
pub const BACKGROUND: Color = Color::ZERO;
pub const ENABLE_GRADIENT_SKY: bool = false;

pub const V_FOV: f64 = 20.0;
pub const LOOKFROM: Point3 = Point3::new(26.0, 3.0, 6.0);
pub const LOOKAT: Point3 = Point3::new(0.0, 2.0, 0.0);
pub const V_UP: DVec3 = Point3::new(0.0, 1.0, 0.0);

pub const DEFOCUS_ANGLE: f64 = 0.0;
pub const FOCUS_DIST: f64 = 10.0;
