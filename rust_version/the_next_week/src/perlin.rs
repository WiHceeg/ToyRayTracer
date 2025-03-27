use rand::{Rng, seq::SliceRandom};

use crate::config_perlin_spheres;
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

    pub fn hash_random_noise(&self, p: Point3) -> f64 {
        let i = (4.0 * p.x) as isize & 255;
        let j = (4.0 * p.y) as isize & 255;
        let k = (4.0 * p.z) as isize & 255;
        self.randfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }

    pub fn trilinear_interpolation_noise(&self, p: Point3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();
        if config_perlin_spheres::HERMITE_CUBIC_SMOOTHED {
            u = u * u * (3.0 - 2.0 * u);
            v = v * v * (3.0 - 2.0 * v);
            w = w * w * (3.0 - 2.0 * w);
        }
        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;
        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.randfloat[index];
                }
            }
        }
        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = di as f64;
                    let j = dj as f64;
                    let k = dk as f64;

                    accum += (i * u + (1.0 - i) * (1.0 - u))
                        * (j * v + (1.0 - j) * (1.0 - v))
                        * (k * w + (1.0 - k) * (1.0 - w))
                        * c[di as usize][dj as usize][dk as usize];
                }
            }
        }
        accum
    }
}
