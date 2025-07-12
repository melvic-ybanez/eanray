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
        // The `& 255` keeps each index within the [0, 255] range. Note that we probably
        // could have just done `% 255`, but `& 255` is faster (though I haven't personally measured).
        // The `* 4` scales the input by 4.
        let i = ((4.0 * p.x) as i32) & 255;
        let j = ((4.0 * p.y) as i32) & 255;
        let k = ((4.0 * p.z) as i32) & 255;

        self.rand_floats
            [self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
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
}
