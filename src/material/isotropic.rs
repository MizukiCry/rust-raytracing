use crate::*;

pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn from_color(color: Vec3) -> Self {
        Self::new(Rc::new(SolidColor::new(color)))
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(record.p, Vec3::random_unit(), ray.time);
        *attenuation = self.albedo.color(record.u, record.v, &record.p);
        true
    }
}