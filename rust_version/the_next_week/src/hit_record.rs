use std::sync::Arc;

use glam::DVec3;

use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::sphere::Sphere;

pub struct HitRecord {
    pub p: Point3,
    pub normal: DVec3, // Sphere hit 时，会计算 normal，用 (p - center) / radius，已经被单位化了
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub u: f64, // the u,v surface coordinates of the ray-object hit point. 纹理坐标，用于纹理映射。
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn with_hit_data(
        t: f64,
        p: Point3,
        (u,v): (f64, f64),
        r: &Ray,
        outward_normal: DVec3,
        mat: Arc<dyn Material>,
    ) -> HitRecord {

        let front_face = r.direction().dot(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p: p,
            normal: normal,
            mat: mat,
            t: t,
            u: u,
            v: v,
            front_face: front_face,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: DVec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
