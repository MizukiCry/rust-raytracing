use crate::*;

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cos: f64, ridx: f64) -> f64 {
        let r0 = (1.0 - ridx) / (1.0 + ridx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);

        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray.direction.unit();
        let cos_theta = (-unit_direction.dot(record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || Self::reflectance(cos_theta, refraction_ratio) > random_f64()
        {
            Vec3::reflect(unit_direction, record.normal)
        } else {
            Vec3::refract(unit_direction, record.normal, refraction_ratio)
        };
        *scattered = Ray::new(record.p, direction, ray.time);
        true
    }
}
