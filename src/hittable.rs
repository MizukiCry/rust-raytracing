use crate::*;

mod sphere;
pub use sphere::*;

mod hittablelist;
pub use hittablelist::*;

mod bvh;
pub use bvh::*;

mod quad;
pub use quad::*;

mod translate;
pub use translate::*;

mod rotate_y;
pub use rotate_y::*;

mod constant_medium;
pub use constant_medium::*;

pub struct HitRecord {
    pub p: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub material: Rc<dyn Material>,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        t: f64,
        u: f64,
        v: f64,
        material: Rc<dyn Material>,
        normal: Vec3,
        front_face: bool,
    ) -> Self {
        Self {
            p,
            t,
            u,
            v,
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
            f64::default(),
            f64::default(),
            Rc::new(Dielectric::default()),
            Vec3::default(),
            bool::default(),
        )
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> &Aabb;

    fn pdf_value(&self, _o: &Vec3, _v: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, _o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
