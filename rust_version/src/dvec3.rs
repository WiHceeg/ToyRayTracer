use glam::DVec3;
use rand::Rng;

use crate::constant;

pub trait DVec3Ext {
    fn random() -> DVec3;
    fn random_range(min: f64, max: f64) -> DVec3;
    fn random_unit() -> DVec3;
    fn random_on_hemisphere(normal: &DVec3) -> DVec3;
    fn near_zero(&self) -> bool;
}

impl DVec3Ext for DVec3 {
    // [0, 1)
    fn random() -> DVec3 {
        let mut rng = rand::rng();
        DVec3::new(rng.random(), rng.random(), rng.random())
    }

    fn random_range(min: f64, max: f64) -> DVec3 {
        let mut rng = rand::rng();
        DVec3::new(
            rng.random_range(min..max),
            rng.random_range(min..max),
            rng.random_range(min..max),
        )
    }

    fn random_unit() -> DVec3 {
        let mut rng = rand::rng();
        loop {
            let p = DVec3::new(rng.random(), rng.random(), rng.random());
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1. {
                return p.normalize();
            }
        }
    }
    
    fn random_on_hemisphere(normal: &DVec3) -> DVec3 {
        let on_unit_sphere = DVec3::random_unit();
        if on_unit_sphere.dot(*normal) > 0. {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    
    fn near_zero(&self) -> bool {
        let s = constant::NEAR_ZERO_THRESHOLD;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}
