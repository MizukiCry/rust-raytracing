use crate::*;

pub struct CosinePDF {
    base: ONB,
}

impl CosinePDF {
    pub fn from(w: &Vec3) -> Self {
        Self { base: ONB::from(w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cos_theta = direction.unit().dot(self.base.w);
        (cos_theta / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.base.local_vec3(&Vec3::random_cosine_direction())
    }
}
