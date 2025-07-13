use crate::core::math;
use crate::core::math::Point;
use serde::{Deserialize, Serialize};

type Perm = Vec<usize>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Perlin {
    rand_floats: Vec<f64>,
    perm_x: Perm,
    perm_y: Perm,
    perm_z: Perm,
}

impl Perlin {
    pub const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut rand_floats = vec![0.0; Self::POINT_COUNT];
        for i in 0..Self::POINT_COUNT {
            rand_floats[i] = math::random_real();
        }

        Self {
            rand_floats,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_floats[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    fn perlin_generate_perm() -> Perm {
        let mut perm = vec![0; Self::POINT_COUNT];
        for i in 0..Self::POINT_COUNT {
            perm[i] = i;
        }
        Self::permute(&mut perm, Self::POINT_COUNT);
        perm
    }

    fn permute(p: &mut Perm, n: usize) {
        for i in ((0 + 1)..n).rev() {
            let target = math::random_int(0, i as i32) as usize;
            let temp = p[i];
            p[i] = p[target];
            p[target] = temp;
        }
    }

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i as usize][j as usize][k as usize]
                }
            }
        }
        accum
    }
}
