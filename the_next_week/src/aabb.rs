use core::f64;
use std::ops::{Index, IndexMut, Add, AddAssign};

use glam::DVec3;

use crate::constant;
use crate::point3::Point3;

use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub const EMPTY: Aabb = Aabb {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: Aabb = Aabb {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    pub fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x: x, y: y, z: z }.pad_to_minimums()
    }

    pub fn new_from_2_points(a: Point3, b: Point3) -> Aabb {
        Aabb {
            x: if a.x < b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },
            y: if a.y < b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },
            z: if a.z < b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        }.pad_to_minimums()
    }

    pub fn new_from_2_strict_ordered_points(a: Point3, b: Point3) -> Aabb {
        Aabb {
            x: Interval::new(a.x, b.x),
            y: Interval::new(a.y, b.y),
            z: Interval::new(a.z, b.z),
        }.pad_to_minimums()
    }

    pub fn new_from_points_vec(points: Vec<Point3>) -> Aabb {
        assert!(points.len() >= 2, "points.len() must >= 2");
        
        let first = points[0];
        let (mut min_x, mut max_x) = (first.x, first.x);
        let (mut min_y, mut max_y) = (first.y, first.y);
        let (mut min_z, mut max_z) = (first.z, first.z);

        for p in points.iter() {
            if p.x < min_x { min_x = p.x; }
            if p.x > max_x { max_x = p.x; }
            if p.y < min_y { min_y = p.y; }
            if p.y > max_y { max_y = p.y; }
            if p.z < min_z { min_z = p.z; }
            if p.z > max_z { max_z = p.z; }
        }

        let aabb = Aabb {
            x: Interval::new(min_x, max_x),
            y: Interval::new(min_y, max_y),
            z: Interval::new(min_z, max_z),
        };
        
        aabb.pad_to_minimums()
    }


    pub fn new_from_merged(box0: Aabb, box1: Aabb) -> Aabb {
        Aabb {
            x: Interval::new_from_merged(box0.x, box1.x),
            y: Interval::new_from_merged(box0.y, box1.y),
            z: Interval::new_from_merged(box0.z, box1.z),
        }.pad_to_minimums()
    }

    pub fn translate(&self, offset: DVec3) -> Aabb {
        *self + offset
    }

    pub fn rotate_y(&self, angle: f64) -> Aabb {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut min = Point3::new(f64::INFINITY, self.y.min, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, self.y.max, f64::NEG_INFINITY);
        
        for i in [0.0, 1.0] {
            for k in [0.0, 1.0] {
                let x = i * self.x.max + (1.0 - i) * self.x.min;
                let z = k * self.z.max + (1.0 - k) * self.z.min;
                let new_x = cos_theta * x - sin_theta * z;
                let new_z = -sin_theta * x + cos_theta * z;
                min.x = min.x.min(new_x);
                min.z = min.z.min(new_z);
                max.x = max.x.max(new_x);
                max.z = max.z.max(new_z);
            }
        }

        Aabb::new_from_2_strict_ordered_points(min, max)
    }


    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();
        for axis in 0..3 {
            let axis_interval = self[axis];
            let axis_dir_inverse = 1.0 / ray_dir[axis];
            let mut t0 = (axis_interval.min - ray_orig[axis]) * axis_dir_inverse;
            let mut t1 = (axis_interval.max - ray_orig[axis]) * axis_dir_inverse;
            if t0 > t1 {
                (t0, t1) = (t1, t0)
            }
            if t0 > ray_t.min {
                ray_t.min = t0;
            }
            if t1 < ray_t.max {
                ray_t.max = t1;
            }
            if ray_t.min >= ray_t.max {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        let (x_size, y_size, z_size) = (self.x.size(), self.y.size(), self.z.size());
        if x_size >= y_size && x_size >= z_size {
            0
        } else if y_size >= z_size {
            1
        } else {
            2
        }
    }

    fn pad_to_minimums(&self) -> Aabb {
        Aabb {
            x: if self.x.size() < constant::MINIMUM_AABB_THICKNESS {
                self.x.expand(constant::MINIMUM_AABB_THICKNESS)
            } else {
                self.x
            },
            y: if self.y.size() < constant::MINIMUM_AABB_THICKNESS {
                self.y.expand(constant::MINIMUM_AABB_THICKNESS)
            } else {
                self.y
            },
            z: if self.z.size() < constant::MINIMUM_AABB_THICKNESS {
                self.z.expand(constant::MINIMUM_AABB_THICKNESS)
            } else {
                self.z
            },
        }
    }
}

impl Index<usize> for Aabb {
    type Output = Interval;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Aabb {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl Add<DVec3> for Aabb {
    type Output = Aabb;

    fn add(self, offset: DVec3) -> Self::Output {
        Aabb {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }
}

impl AddAssign<DVec3> for Aabb {
    fn add_assign(&mut self, offset: DVec3) {
        self.x += offset.x;
        self.y += offset.y;
        self.z += offset.z;
    }
}
