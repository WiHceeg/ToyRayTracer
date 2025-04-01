use std::sync::Arc;

use glam::DVec3;


use crate::aabb::Aabb;
use crate::constant;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
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
    unit_normal: DVec3,
    D: f64, // 平面方程的常数项
}

impl Quad {
    pub fn new(Q: Point3, u: DVec3, v: DVec3, mat: Arc<dyn Material>) -> Quad {
        let n = u.cross(v);
        let unit_normal = n.normalize();
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
            unit_normal: unit_normal,
            D: unit_normal.dot(Q),
        }
    }

    pub fn cuboid(a: Point3, b: Point3, mat: Arc<dyn Material>) -> HittableList {
        let mut sides = HittableList::new();
        let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = DVec3::new(max.x - min.x, 0.0, 0.0);
        let dy = DVec3::new(0.0, max.y - min.y, 0.0);
        let dz = DVec3::new(0.0, 0.0, max.z - min.z);

        sides.add(Arc::new(Quad::new(Point3::new(min.x, min.y, min.z), dx, dy, mat.clone())));
        sides.add(Arc::new(Quad::new(Point3::new(max.x, min.y, max.z), -dz, dy, mat.clone())));
        sides.add(Arc::new(Quad::new(Point3::new(max.x, min.y, min.z), -dx, dy, mat.clone())));
        sides.add(Arc::new(Quad::new(Point3::new(min.x, min.y, max.z), dz, dy, mat.clone())));
        sides.add(Arc::new(Quad::new(Point3::new(min.x, max.y, max.z), dx, -dz, mat.clone())));
        sides.add(Arc::new(Quad::new(Point3::new(min.x, min.y, min.z), dx, dz, mat.clone())));
        
        sides
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
        self.unit_normal
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
    unit_normal: DVec3,
    D: f64, // 平面方程的常数项
}

impl Tri {
    pub fn new(Q: Point3, u: DVec3, v: DVec3, mat: Arc<dyn Material>) -> Tri {
        let n = u.cross(v);
        let unit_normal = n.normalize();
        let w = n / n.length_squared();

        Tri {
            Q: Q,
            u: u,
            v: v,
            w: w,
            mat: mat,
            bbox: Aabb::new_from_points_vec(vec![Q, Q + u, Q + v]),
            unit_normal: unit_normal,
            D: unit_normal.dot(Q),
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
        self.unit_normal
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


pub struct Ellipse {
    center: Point3,
    a: DVec3,   // a, b 分别是长轴和短轴的向量，互相垂直
    b: DVec3,
    w: DVec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    unit_normal: DVec3,
    D: f64, // 平面方程的常数项
}

impl Ellipse {
    pub fn new(center: Point3, a: DVec3, b: DVec3, mat: Arc<dyn Material>) -> Ellipse {
        let n = a.cross(b);
        let unit_normal = n.normalize();
        let w = n / n.length_squared();

        let bbox = Aabb::new(
            Interval::new(center.x - (a.x * a.x + b.x * b.x).sqrt(), center.x + (a.x * a.x + b.x * b.x).sqrt()),
            Interval::new(center.y - (a.y * a.y + b.y * b.y).sqrt(), center.y + (a.y * a.y + b.y * b.y).sqrt()),
            Interval::new(center.z - (a.z * a.z + b.z * b.z).sqrt(), center.z + (a.z * a.z + b.z * b.z).sqrt()),
        );

        Ellipse {
            center: center,
            a: a,
            b: b,
            w: w,
            mat: mat,
            bbox: bbox,
            unit_normal: unit_normal,
            D: unit_normal.dot(center),
        }
    }
}


impl Hittable for Ellipse {

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        <Self as Shape>::hit(&self, r, ray_t)
    }
}


impl Shape for Ellipse {
    fn get_normal(&self) -> DVec3 {
        self.unit_normal
    }

    fn get_w(&self) -> DVec3 {
        self.w
    }

    fn get_D(&self) -> f64 {
        self.D
    }

    fn get_Q(&self) -> Point3 {
        self.center
    }

    fn get_u(&self) -> DVec3 {
        self.a
    }

    fn get_v(&self) -> DVec3 {
        self.b
    }

    fn get_mat_clone(&self) -> Arc<dyn Material> {
        self.mat.clone()
    }
    
    fn alpha_beta_hit_uv(&self, alpha: f64, beta: f64) -> Option<(f64, f64)> {
        if alpha * alpha + beta * beta > 1.0 {
            return None;
        }
        Some((alpha / 2.0 + 0.5, beta / 2.0 + 0.5))
    }
}


pub struct Annulus {
    center: Point3,
    outer_vector: DVec3, 
    inner_vector: DVec3,
    outer_radius: f64,
    inner_radius: f64,
    w: DVec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    unit_normal: DVec3,
    D: f64, // 平面方程的常数项
}

impl Annulus {
    pub fn new(center: Point3, outer_vector: DVec3, inner_vector: DVec3, mat: Arc<dyn Material>) -> Annulus {
        let outer_radius = outer_vector.length();
        let inner_radius = inner_vector.length();
        let n = outer_vector.cross(inner_vector);
        let unit_normal = n.normalize();
        let w = n / n.length_squared();
        let bbox = Aabb::new(
            Interval::new(center.x - outer_radius * (1.0 - unit_normal.x * unit_normal.x).sqrt(), center.x + outer_radius * (1.0 - unit_normal.x * unit_normal.x).sqrt()),
            Interval::new(center.y - outer_radius * (1.0 - unit_normal.y * unit_normal.y).sqrt(), center.y + outer_radius * (1.0 - unit_normal.y * unit_normal.y).sqrt()),
            Interval::new(center.z - outer_radius * (1.0 - unit_normal.z * unit_normal.z).sqrt(), center.z + outer_radius * (1.0 - unit_normal.z * unit_normal.z).sqrt()),
        );

        Annulus {
            center: center,
            outer_vector: outer_vector,
            inner_vector: inner_vector,
            outer_radius: outer_radius,
            inner_radius: inner_radius,
            w: w,
            mat: mat,
            bbox: bbox,
            unit_normal: unit_normal,
            D: unit_normal.dot(center),
        }        
    }
}



impl Hittable for Annulus {

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        <Self as Shape>::hit(&self, r, ray_t)
    }
}


impl Shape for Annulus {
    fn get_normal(&self) -> DVec3 {
        self.unit_normal
    }

    fn get_w(&self) -> DVec3 {
        self.w
    }

    fn get_D(&self) -> f64 {
        self.D
    }

    fn get_Q(&self) -> Point3 {
        self.center
    }

    fn get_u(&self) -> DVec3 {
        self.outer_vector
    }

    fn get_v(&self) -> DVec3 {
        self.inner_vector
    }

    fn get_mat_clone(&self) -> Arc<dyn Material> {
        self.mat.clone()
    }
    
    fn alpha_beta_hit_uv(&self, alpha: f64, beta: f64) -> Option<(f64, f64)> {
        let distance_squared = (alpha * self.outer_vector + beta * self.inner_vector).length_squared();
        if distance_squared < self.inner_radius * self.inner_radius || self.outer_radius * self.outer_radius < distance_squared {
            return None;
        }
        Some((alpha / 2.0 + 0.5, beta / 2.0 + 0.5))
    }
}