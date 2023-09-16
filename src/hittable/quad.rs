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

    pub fn new_box(a: Vec3, b: Vec3, material: Rc<dyn Material>) -> HittableList {
        let mut sides = HittableList::default();

        let min_x = a.x.min(b.x);
        let min_y = a.y.min(b.y);
        let min_z = a.z.min(b.z);
        let max_x = a.x.max(b.x);
        let max_y = a.y.max(b.y);
        let max_z = a.z.max(b.z);

        let dx = Vec3::new(max_x - min_x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max_y - min_y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max_z - min_z);
        sides.add(Rc::new(Self::new(
            Vec3::new(min_x, min_y, max_z),
            dx,
            dy,
            Rc::clone(&material),
        )));
        sides.add(Rc::new(Self::new(
            Vec3::new(max_x, min_y, max_z),
            -dz,
            dy,
            Rc::clone(&material),
        )));
        sides.add(Rc::new(Self::new(
            Vec3::new(max_x, min_y, min_z),
            -dx,
            dy,
            Rc::clone(&material),
        )));
        sides.add(Rc::new(Self::new(
            Vec3::new(min_x, min_y, min_z),
            dz,
            dy,
            Rc::clone(&material),
        )));
        sides.add(Rc::new(Self::new(
            Vec3::new(min_x, max_y, max_z),
            dx,
            -dz,
            Rc::clone(&material),
        )));
        sides.add(Rc::new(Self::new(
            Vec3::new(min_x, min_y, min_z),
            dx,
            dz,
            Rc::clone(&material),
        )));

        sides
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
