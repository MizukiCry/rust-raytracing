use crate::*;

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    material: Rc<dyn Material>,
    bounding_box: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Rc<dyn Material>) -> Self {
        let n = u.cross(v);
        let normal = n.unit();
        Self {
            q,
            u,
            v,
            material,
            bounding_box: Aabb::from_vec3(&q, &(q + u + v)).pad(),
            normal,
            d: normal.dot(q),
            w: n / n.dot(n),
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let denom = self.normal.dot(ray.direction);
        if equal(denom, 0.0) {
            return false;
        }

        let t = (self.d - self.normal.dot(ray.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = ray.at(t);
        let phv = intersection - self.q;
        let alpha = self.w.dot(phv.cross(self.v));
        let beta = self.w.dot(self.u.cross(phv));

        let range = Interval::new(0.0, 1.0);
        if !range.contains(alpha) || !range.contains(beta) {
            return false;
        }

        record.p = intersection;
        record.t = t;
        record.u = alpha;
        record.v = beta;
        record.material = Rc::clone(&self.material);
        record.set_face_normal(ray, self.normal);

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
