
use glam::DVec3;

use crate::point3::Point3;

pub struct HitRecord {
    pub p: Point3,
    pub normal: DVec3,
    pub t: f64,
}