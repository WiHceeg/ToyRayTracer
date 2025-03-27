use rand::{seq::SliceRandom, Rng};

use crate::constant::PERLIN_POINT_COUNT;
use crate::point3::Point3;
pub struct Perlin {
    randfloat: [f64; PERLIN_POINT_COUNT],
    perm_x: [usize; PERLIN_POINT_COUNT],
    perm_y: [usize; PERLIN_POINT_COUNT],
    perm_z: [usize; PERLIN_POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut rng = rand::rng();
        let randfloat: [f64; PERLIN_POINT_COUNT] = rng.random();

        let mut perlin_generate_perm = || {
            let mut perm: [usize; PERLIN_POINT_COUNT] = std::array::from_fn(|x| x);
            perm.shuffle(&mut rng);
            perm
        };

        Perlin {
            randfloat: randfloat,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = (4.0 * p.x) as isize & 255;
        let j = (4.0 * p.y) as isize & 255;
        let k = (4.0 * p.z) as isize & 255;
        self.randfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }

}