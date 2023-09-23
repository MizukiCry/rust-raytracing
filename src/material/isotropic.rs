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
    fn scatter(&self, _ray: &Ray, record: &HitRecord, srecord: &mut ScatterRecord) -> bool {
        srecord.attenuation = self.albedo.color(record.u, record.v, &record.p);
        srecord.pdf = Some(Rc::new(SpherePDF::default()));
        true
    }

    fn scattering_pdf(&self, _ray: &Ray, _record: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
