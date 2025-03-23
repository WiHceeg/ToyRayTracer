
use crate::hit_record::HitRecord;
use crate::interval::Interval;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}