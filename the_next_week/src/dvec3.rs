use std::ops::Add;

use glam::DVec3;

use crate::aabb::Aabb;
use crate::constant;
use crate::random_number_generator::{random, random_range};


pub trait DVec3Ext {
    fn random() -> DVec3;
    fn random_range(min: f64, max: f64) -> DVec3;
    fn random_unit() -> DVec3;
    fn random_on_hemisphere(normal: DVec3) -> DVec3;
    fn random_in_unit_disk() -> DVec3;
    fn near_zero(&self) -> bool;
}

impl DVec3Ext for DVec3 {
    // [0, 1)
    fn random() -> DVec3 {
        DVec3::new(random(), random(), random())
    }

    fn random_range(min: f64, max: f64) -> DVec3 {
        DVec3::new(
            random_range(min..max),
            random_range(min..max),
            random_range(min..max),
        )
    }

    fn random_unit() -> DVec3 {
        loop {
            let p = DVec3::new(random_range(-1.0..1.0), random_range(-1.0..1.0), random_range(-1.0..1.0));
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1. {
                return p.normalize();
            }
        }
    }
    
    fn random_on_hemisphere(normal: DVec3) -> DVec3 {
        let on_unit_sphere = DVec3::random_unit();
        if on_unit_sphere.dot(normal) > 0. {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    fn near_zero(&self) -> bool {
        let s = constant::NEAR_ZERO_THRESHOLD;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
    
    fn random_in_unit_disk() -> DVec3 {
        loop {
            let p = DVec3::new(random_range(-1.0..1.0), random_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}


impl Add<Aabb> for DVec3 {
    type Output = Aabb;

    fn add(self, aabb: Aabb) -> Self::Output {
        aabb + self
    }
}