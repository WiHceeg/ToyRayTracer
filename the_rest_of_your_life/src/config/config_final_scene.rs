use crate::color::Color;
use crate::point3::Point3;
use glam::DVec3;

pub const INPUT_POINT_SCALE: f64 = 0.2;

pub const ASPECT_RATIO: f64 = 1.0;

pub const IMAGE_WIDTH_HD: usize = 800;
pub const IMAGE_WIDTH_LD: usize = 400;
pub const SAMPLES_PER_PIXEL_HD: usize = 10000; // Count of random samples for each pixel
pub const SAMPLES_PER_PIXEL_LD: usize = 250; // Count of random samples for each pixel
pub const MAX_DEPTH_HD: usize = 40; // Maximum number of ray bounces into scene
pub const MAX_DEPTH_LD: usize = 4; // Maximum number of ray bounces into scene

pub const BACKGROUND: Color = Color::ZERO;
pub const ENABLE_GRADIENT_SKY: bool = false;

pub const V_FOV: f64 = 40.0;
pub const LOOKFROM: Point3 = Point3::new(478.0, 278.0, -600.0);
pub const LOOKAT: Point3 = Point3::new(278.0, 278.0, 0.0);
pub const V_UP: DVec3 = Point3::new(0.0, 1.0, 0.0);

pub const DEFOCUS_ANGLE: f64 = 0.0;
pub const FOCUS_DIST: f64 = 10.0;
