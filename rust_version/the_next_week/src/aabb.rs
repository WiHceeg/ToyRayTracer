use std::ops::{Index, IndexMut};

use glam::DVec3;

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

    pub fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x: x, y: y, z: x }
    }

    pub fn new_from_points(a: Point3, b: Point3) -> Aabb {
        Aabb {
            x: if a.x < b.x {Interval::new(a.x, b.x)} else {Interval::new(b.x, a.x)},
            y: if a.y < b.y {Interval::new(a.y, b.y)} else {Interval::new(b.y, a.y)},
            z: if a.z < b.z {Interval::new(a.z, b.z)} else {Interval::new(b.z, a.z)}, 
        }
    }

    pub fn new_from_merged(box0: Aabb, box1: Aabb) -> Aabb {
        Aabb {
            x: Interval::new_from_merged(box0.x, box1.x),
            y: Interval::new_from_merged(box0.y, box1.y),
            z: Interval::new_from_merged(box0.z, box1.z),
        }
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
            if t0 > ray_t.min {ray_t.min = t0;}
            if t1 < ray_t.max {ray_t.max = t1;}
            if ray_t.min <= ray_t.max {
                return false;
            }
        }
        true
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