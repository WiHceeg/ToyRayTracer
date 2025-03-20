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

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn ray_color(&self) -> Color {
        // 没击中，背景色，这里可以理解成天空的颜色
        let unit_direction = self.dir.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * config::SKY_GRADIENT
    }
}
