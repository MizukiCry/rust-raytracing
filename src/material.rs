use crate::utils::*;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub fn defaule_material() -> Box<dyn Material> {
    Box::new(Lambertian::default())
}

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
        _ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut direction = record.normal + Vec3::random_unit();
        if is_zero_vec3(direction) {
            direction = record.normal;
        }
        *scattered = Ray::new(record.p, direction);
        *attenuation = self.albedo;
        true
    }
}

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
    ) -> bool {
        let reflected = Vec3::reflect(ray.direction.unit(), record.normal);
        *scattered = Ray::new(record.p, reflected + self.fuzz * Vec3::random_unit());
        *attenuation = self.albedo;
        scattered.direction.dot(record.normal).is_sign_positive()
    }
}
