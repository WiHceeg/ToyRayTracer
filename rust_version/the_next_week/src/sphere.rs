use std::sync::Arc;

use glam::DVec3;

use crate::aabb::Aabb;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;

pub struct Sphere {
    center: Ray,    // 默认场景假设所有动画从 t=0 开始，因此 center 的 tm 都是默认 0
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new_static(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        let rvec = DVec3::splat(radius);
        Sphere {
            center: Ray::new(static_center, DVec3::ZERO),
            radius: radius.max(0.),
            mat: mat,
            bbox: Aabb::new_from_points(static_center - rvec, static_center + rvec),
        }
    }

    pub fn new_moving(start_center: Point3, end_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        let center = Ray::new(start_center, end_center - start_center);
        let rvec = DVec3::splat(radius);
        let start_box = Aabb::new_from_points(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let end_box = Aabb::new_from_points(center.at(1.0) - rvec, center.at(1.0) + rvec);
        Sphere {
            center: center,
            radius: radius.max(0.),
            mat: mat,
            bbox: Aabb::new_from_merged(start_box, end_box),
        }

    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let b = -2.0 * r.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-b - sqrtd) / (2. * a);
        if !ray_t.surrounds(root) {
            root = (-b + sqrtd) / (2. * a);
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - current_center) / self.radius; // 单位化

        let rec = HitRecord::with_hit_data(root, p, r, outward_normal, self.mat.clone());
        Some(rec)
    }
    
    fn bounding_box(&self) -> crate::aabb::Aabb {
        self.bbox
    }
}
