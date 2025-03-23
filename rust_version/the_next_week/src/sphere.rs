use std::sync::Arc;

use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius.max(0.),
            mat: mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let b = -2.0 * r.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-b - discriminant.sqrt()) / (2. * a);
        if !ray_t.surrounds(root) {
            root = (-b + discriminant.sqrt()) / (2. * a);
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;   // 单位化
        
        let rec = HitRecord::with_hit_data(root, p, r, &outward_normal, self.mat.clone());
        Some(rec)
    }
}
