use crate::*;

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Default for Metal {
    fn default() -> Self {
        Self::new(Vec3::default(), 0.1)
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        let reflected = Vec3::reflect(ray.direction.unit(), record.normal);
        *scattered = Ray::new(
            record.p,
            reflected + self.fuzz * Vec3::random_unit(),
            ray.time,
        );
        *attenuation = self.albedo;
        scattered.direction.dot(record.normal).is_sign_positive()
    }
}
