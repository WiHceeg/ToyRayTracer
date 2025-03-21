
use glam::DVec3;

use crate::point3::Point3;
use crate::ray::Ray;

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &DVec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = r.direction().dot(*outward_normal) < 0.;
        self.normal = if self.front_face {*outward_normal} else {-outward_normal};
    }
}