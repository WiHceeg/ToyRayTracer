
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

pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    // let t =  hit_sphere(&Point3::new(0., 0., -1.), 0.5, r);
    // if t > 0. {
    //     let N = (r.at(t) - DVec3::new(0.,0.,-1.)).normalize();
    //     return 0.5 * (N + DVec3::splat(1.0));
    // }

    if let Some(rec) = world.hit(r, Interval::new(0., f64::INFINITY)) {
        return 0.5 * (rec.normal + DVec3::splat(1.0));
    }

    // 没击中，背景色，这里可以理解成天空的颜色
    let unit_direction = r.dir.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * config::SKY_GRADIENT
}
