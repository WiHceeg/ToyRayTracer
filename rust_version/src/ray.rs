
use std::f64;

use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::Point3;
use crate::color::Color;
use crate::config;
use glam::DVec3;

pub struct Ray {
    orig: Point3,
    dir: DVec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: DVec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &DVec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

