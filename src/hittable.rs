use crate::utils::*;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub t: f64,
    pub material: Rc<dyn Material>,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        t: f64,
        material: Rc<dyn Material>,
        normal: Vec3,
        front_face: bool,
    ) -> Self {
        Self {
            p,
            t,
            material,
            normal,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal).is_sign_negative();
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new(
            Vec3::default(),
            f64::default(),
            Rc::new(Lambertian::default()),
            Vec3::default(),
            bool::default(),
        )
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Rc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new(Vec::default())
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_t = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_t, record) {
                hit_anything = true;
                closest_t = record.t;
            }
        }

        hit_anything
    }
}
