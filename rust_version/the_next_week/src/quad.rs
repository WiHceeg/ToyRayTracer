use std::sync::Arc;

use glam::DVec3;

use crate::aabb::Aabb;
use crate::constant;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;

pub struct Quad {
    Q: Point3,
    u: DVec3,
    v: DVec3,
    w: DVec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: DVec3,
    D: f64,
}

impl Quad {
    pub fn new(Q: Point3, u: DVec3, v: DVec3, mat: Arc<dyn Material>) -> Quad {
        let n = u.cross(v);
        let normal = n.normalize();
        let w = n / n.length_squared();
        let bbox_diagonal1 = Aabb::new_from_points(Q, Q + u + v);
        let bbox_diagonal2 = Aabb::new_from_points(Q + u, Q + v);
        Quad {
            Q: Q,
            u: u,
            v: v,
            w: w,
            mat: mat,
            bbox: Aabb::new_from_merged(bbox_diagonal1, bbox_diagonal2),
            normal: normal,
            D: normal.dot(Q),
        }
    }
}

impl Hittable for Quad {

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denominator = self.normal.dot(r.direction());
        if denominator.abs() < constant::NEAR_ZERO_THRESHOLD {
            return None;
        }
        let t = (self.D - self.normal.dot(r.origin())) / denominator;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let planar_hip_point_vector = intersection - self.Q;
        let alpha = self.w.dot(planar_hip_point_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hip_point_vector));
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }
        Some(HitRecord::with_hit_data(t, intersection, r, self.normal, self.mat.clone()))
    }
}