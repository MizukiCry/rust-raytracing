use crate::utils::*;
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Vec3::default(), f64::default())
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant.is_sign_negative() {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        record.p = ray.at(root);
        record.t = root;
        record.set_face_normal(ray, (record.p - self.center) / self.radius);

        true
    }
}