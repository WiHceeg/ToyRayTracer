use core::f64;
use std::sync::Arc;

use glam::DVec3;

use crate::aabb::Aabb;
use crate::color::Color;
use crate::constant;
use crate::dvec3::DVec3Ext;
use crate::hittable::Hittable;
use crate::hit_record::HitRecord;
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::texture::Texture;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new_from_solid_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_from_solid_color(albedo)),
        }
    }

    pub fn new_from_texture(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_from_texture(tex)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // 进入
        let Some(mut rec1) = self.boundary.hit(r, Interval::UNIVERSE) else {
            return None;
        };
        // 离开
        let Some(mut rec2) = self.boundary.hit(r, Interval::new(rec1.t + constant::RAY_MIN_DISTANCE, f64::INFINITY)) else {
            return None;
        };

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();
        if hit_distance > distance_inside_boundary {
            return None;
        }

        let hit_t = rec1.t + hit_distance / ray_length;
        Some(HitRecord::with_hit_data(
            hit_t,
            r.at(hit_t),
            (0.0, 0.0),
            r,
            DVec3::random_unit(), // 随机的法线
            self.phase_function.clone(),
        ))


    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}