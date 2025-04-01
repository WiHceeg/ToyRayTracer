
use std::f64;

use crate::Point3;
use glam::DVec3;

pub struct Ray {
    orig: Point3,
    dir: DVec3,
    tm: f64,    // 发射的时刻
}

impl Ray {
    pub fn new(origin: Point3, direction: DVec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            tm: 0.0,    // 发射的时刻, 默认是 0
        }
    }

    pub fn new_with_time(origin: Point3, direction: DVec3, time: f64) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            tm: time,
        }        
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> DVec3 {
        self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

