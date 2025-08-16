use crate::core::math;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Point, Vec3D};
use serde::{Deserialize, Serialize};

type Perm = Vec<usize>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Perlin {
    rand_vecs: Vec<UnitVec3D>,
    perm_x: Perm,
    perm_y: Perm,
    perm_z: Perm,
}

impl Perlin {
    pub(crate) const POINT_COUNT: usize = 256;

    pub(crate) fn new() -> Self {
        let mut rand_vecs = vec![UnitVec3D(Vec3D::zero()); Self::POINT_COUNT];
        for i in 0..Self::POINT_COUNT {
            rand_vecs[i] = Vec3D::random_range(-1.0, 1.0).to_unit();
        }

        Self {
            rand_vecs,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub(crate) fn noise(&self, p: &Point) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = vec![vec![vec![UnitVec3D(Vec3D::zero()); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vecs[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                        .clone()
                }
            }
        }

        Self::perlin_interpolation(&c, u, v, w)
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

    fn perlin_interpolation(c: &Vec<Vec<Vec<UnitVec3D>>>, u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3D::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * c[i as usize][j as usize][k as usize].0.dot(&weight_v)
                }
            }
        }
        accum
    }

    pub(crate) fn turbulence(&self, p: &Point, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }
}
