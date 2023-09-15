use crate::*;

pub const POINT_COUNT: usize = 256;

pub struct Perlin {
    random_vec3: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut random_vec3 = [Vec3::default(); POINT_COUNT];
        for x in &mut random_vec3 {
            *x = Vec3::random_range(-1.0, 1.0).unit();
        }

        Self {
            random_vec3,
            perm_x: Self::random_permutation(),
            perm_y: Self::random_permutation(),
            perm_z: Self::random_permutation(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut accum = 0.0;

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    accum += (di as f64 * uu + (1.0 - di as f64) * (1.0 - uu))
                        * (dj as f64 * vv + (1.0 - dj as f64) * (1.0 - vv))
                        * (dk as f64 * ww + (1.0 - dk as f64) * (1.0 - ww))
                        * self.random_vec3[self.perm_x[((i + di) & 255) as usize]
                            ^ self.perm_y[((j + dj) & 255) as usize]
                            ^ self.perm_z[((k + dk) & 255) as usize]]
                            .dot(Vec3::new(u - di as f64, v - dj as f64, w - dk as f64));
                }
            }
        }

        accum
    }

    pub fn turb(&self, mut p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&p);
            weight *= 0.5;
            p *= 2.0;
        }

        accum.abs()
    }

    fn random_permutation() -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for (i, x) in p.iter_mut().enumerate() {
            *x = i;
        }
        for i in 1..POINT_COUNT {
            let j = random_range_i32(0, i as i32) as usize;
            if i != j {
                (p[i], p[j]) = (p[j], p[i])
            }
        }
        p
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn color(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        let s = self.scale * *p;
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (s.z + 10.0 * self.noise.turb(s, 7)).sin())
    }
}
