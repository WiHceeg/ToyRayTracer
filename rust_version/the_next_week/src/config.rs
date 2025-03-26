use glam::DVec3;

use crate::color::Color;
use crate::point3::Point3;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: usize = 400;
pub const SAMPLES_PER_PIXEL: usize = 100; // Count of random samples for each pixel
pub const MAX_DEPTH: usize = 50; // Maximum number of ray bounces into scene

pub const V_FOV: f64 = 20.0;
pub const LOOKFROM: Point3 = Point3::new(13.0, 2.0, 3.0);
pub const LOOKAT: Point3 = Point3::new(0.0, 0.0, 0.0);
pub const V_UP: DVec3 = Point3::new(0.0, 1.0, 0.0);

pub const DEFOCUS_ANGLE: f64 = 0.6;
pub const FOCUS_DIST: f64 = 10.0;

pub const SKY_GRADIENT: Color = Color::new(0.5, 0.7, 1.0);

pub const ENABLE_BVH: bool = true;