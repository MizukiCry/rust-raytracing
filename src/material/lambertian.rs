use crate::*;

pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn from_color(color: Vec3) -> Self {
        Self::new(Rc::new(SolidColor::new(color)))
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self::new(Rc::new(SolidColor::new(Vec3::default())))
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
        let mut direction = Vec3::random_on_hemisphere(record.normal);
        if is_zero_vec3(direction) {
            direction = record.normal;
        }
        *scattered = Ray::new(record.p, direction, ray.time);
        *attenuation = self.albedo.color(record.u, record.v, &record.p);
        true
    }

    fn scattering_pdf(&self, _ray: &Ray, record: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = record.normal.dot(scattered.direction.unit());
        if cos_theta.is_sign_positive() {
            cos_theta / PI
        } else {
            1.0 / (2.0 * PI)
        }
    }
}
