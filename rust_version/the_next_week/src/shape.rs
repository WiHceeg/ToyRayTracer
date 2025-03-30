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


pub trait Shape: Hittable {
    
    fn get_normal(&self) -> DVec3;
    fn get_w(&self) -> DVec3;
    fn get_D(&self) -> f64;
    fn get_Q(&self) -> Point3;
    fn get_u(&self) -> DVec3;
    fn get_v(&self) -> DVec3;
    fn get_mat_clone(&self) -> Arc<dyn Material>;

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denominator = self.get_normal().dot(r.direction());
        if denominator.abs() < constant::NEAR_ZERO_THRESHOLD {
            return None;
        }
        let t = (self.get_D() - self.get_normal().dot(r.origin())) / denominator;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let planar_hip_point_vector = intersection - self.get_Q();
        let alpha = self.get_w().dot(planar_hip_point_vector.cross(self.get_v()));
        let beta = self.get_w().dot(self.get_u().cross(planar_hip_point_vector));

        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }

        if let Some(uv) = self.alpha_beta_hit_uv(alpha, beta) {
            Some(HitRecord::with_hit_data(t, intersection, uv, r, self.get_normal(), self.get_mat_clone()))
        } else {
            None
        }
    }

    /// 输入以 self.u, self.v 基向量为坐标轴的坐标，输出纹理坐标
    fn alpha_beta_hit_uv(&self, alpha: f64, beta: f64) -> Option<(f64, f64)>;

}


pub struct Quad {
    Q: Point3,
    u: DVec3,
    v: DVec3,
    w: DVec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: DVec3,
    D: f64, // 平面方程的常数项
}

impl Quad {
    pub fn new(Q: Point3, u: DVec3, v: DVec3, mat: Arc<dyn Material>) -> Quad {
        let n = u.cross(v);
        let normal = n.normalize();
        let w = n / n.length_squared();
        let bbox_diagonal1 = Aabb::new_from_2_points(Q, Q + u + v);
        let bbox_diagonal2 = Aabb::new_from_2_points(Q + u, Q + v);
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
        <Self as Shape>::hit(&self, r, ray_t)
    }
}

impl Shape for Quad {
    fn get_normal(&self) -> DVec3 {
        self.normal
    }

    fn get_w(&self) -> DVec3 {
        self.w
    }

    fn get_D(&self) -> f64 {
        self.D
    }

    fn get_Q(&self) -> Point3 {
        self.Q
    }

    fn get_u(&self) -> DVec3 {
        self.u
    }

    fn get_v(&self) -> DVec3 {
        self.v
    }

    fn get_mat_clone(&self) -> Arc<dyn Material> {
        self.mat.clone()
    }
    
    fn alpha_beta_hit_uv(&self, alpha: f64, beta: f64) -> Option<(f64, f64)> {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }
        Some((alpha, beta))
    }
}


pub struct Tri {
    Q: Point3,
    u: DVec3,
    v: DVec3,
    w: DVec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: DVec3,
    D: f64, // 平面方程的常数项
}

impl Tri {
    pub fn new(Q: Point3, u: DVec3, v: DVec3, mat: Arc<dyn Material>) -> Tri {
        let n = u.cross(v);
        let normal = n.normalize();
        let w = n / n.length_squared();

        Tri {
            Q: Q,
            u: u,
            v: v,
            w: w,
            mat: mat,
            bbox: Aabb::new_from_points_vec(vec![Q, Q + u, Q + v]),
            normal: normal,
            D: normal.dot(Q),
        }
    }
}

impl Hittable for Tri {

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        <Self as Shape>::hit(&self, r, ray_t)
    }
}



impl Shape for Tri {
    fn get_normal(&self) -> DVec3 {
        self.normal
    }

    fn get_w(&self) -> DVec3 {
        self.w
    }

    fn get_D(&self) -> f64 {
        self.D
    }

    fn get_Q(&self) -> Point3 {
        self.Q
    }

    fn get_u(&self) -> DVec3 {
        self.u
    }

    fn get_v(&self) -> DVec3 {
        self.v
    }

    fn get_mat_clone(&self) -> Arc<dyn Material> {
        self.mat.clone()
    }
    
    fn alpha_beta_hit_uv(&self, alpha: f64, beta: f64) -> Option<(f64, f64)> {
        if alpha < 0.0 || beta < 0.0 || alpha + beta > 1.0 {
            return None;
        }
        Some((alpha, beta))
    }
}