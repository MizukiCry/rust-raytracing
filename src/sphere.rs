use crate::utils::*;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<Box<dyn Material>>,
    pub is_moving: bool,
    pub moving_direction: Vec3,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<Box<dyn Material>>) -> Self {
        Self {
            center,
            radius,
            material,
            is_moving: false,
            moving_direction: Vec3::default(),
        }
    }

    pub fn new_moving(
        center1: Vec3,
        center2: Vec3,
        radius: f64,
        material: Rc<Box<dyn Material>>,
    ) -> Self {
        Self {
            center: center1,
            radius,
            material,
            is_moving: true,
            moving_direction: center2 - center1,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        if self.is_moving {
            self.center + time * self.moving_direction
        } else {
            self.center
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Vec3::default(), f64::default(), Rc::new(default_material()))
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let center = self.center(ray.time);
        let oc = ray.origin - center;
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
        record.material = Rc::clone(&self.material);
        record.set_face_normal(ray, (record.p - center) / self.radius);

        true
    }
}
