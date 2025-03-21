use std::sync::Arc;

use crate::hit_record::{self, HitRecord};
use crate::hittable::Hittable;
use crate::point3::Point3;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_record = Some(temp_rec);
            }
        }
        hit_record
    }
}