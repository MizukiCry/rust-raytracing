use crate::*;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self::new(Vec3::default())
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut direction = record.normal + Vec3::random_unit();
        if is_zero_vec3(direction) {
            direction = record.normal;
        }
        *scattered = Ray::new(record.p, direction, ray.time);
        *attenuation = self.albedo;
        true
    }
}
