use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::new(Vec3::default(), Vec3::default(), f64::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::equal_vec3;

    use super::*;
    #[test]
    fn basic_test() {
        let r = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 0.0, 0.0), 0.0);
        assert!(equal_vec3(r.at(2.0), Vec3::new(3.0, 1.0, 1.0)));
    }
}
