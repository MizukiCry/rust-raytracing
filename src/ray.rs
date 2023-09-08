use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::new(Vec3::default(), Vec3::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::equal_vec3;

    use super::*;
    #[test]
    fn basic_test() {
        let r = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        assert!(equal_vec3(r.at(2.0), Vec3::new(3.0, 1.0, 1.0)));
    }
}
