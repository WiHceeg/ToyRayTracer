use glam::DVec3;

use crate::interval::Interval;



struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}


impl Aabb {

    pub fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x: x, y: y, z: x }
    }


}