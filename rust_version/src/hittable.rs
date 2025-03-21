
use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}